use crate::config::AppConfig;
use crate::repository::{error::RepositoryError, UserRepository};
use crate::types::users::{
    generate_random_salt, update_user_account, UserCredentials, UserPasswordsPair,
};
use crate::types::users::{UserAccountPut, UserGet, UserPost};
use chrono::Utc;
use common_types::UserIdentifiers;

use async_trait::async_trait;
use bcrypt::{hash_with_salt, verify, DEFAULT_COST};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use uuid::Uuid;

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("SQLX error: {error}");
        let db_error = error.into_database_error().map(|db_err| db_err.kind());
        match db_error {
            Some(err_kind) => RepositoryError::SQLXDatabase(err_kind),
            None => RepositoryError::SQLXOther,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    pool: PgPool,

    // queries
    get_user_by_id_query: String,
    get_user_passwd_and_role_query: String,
}

impl PgUserRepository {
    pub async fn new(pool: PgPool) -> Self {
        let get_user_by_id_query = r#"
            SELECT 
                ul.user_id user_id,
                ul.email email,
                ua.phone_number phone_number,
                ua.first_name first_name,
                ua.last_name last_name, 
                ua.created_at created_at,
                ua.updated_at updated_at
            FROM user_log_infos ul
            INNER JOIN user_accounts ua ON ul.user_id = ua.user_id
            WHERE ul.user_id = $1
        "#
        .to_owned();

        let get_user_passwd_and_role_query =
            r#"SELECT password_hashed, password_salt, user_id, email, role FROM user_log_infos WHERE email = $1"#.to_owned();

        Self {
            pool,
            get_user_by_id_query,
            get_user_passwd_and_role_query,
        }
    }

    pub async fn from_config(config: &AppConfig) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(config.pg_max_conn)
            .connect(&config.pg_url)
            .await
            .expect("Failed to create PostgreSQL pool.");
        tracing::info!("Initiated PosgreSQL pool");
        Self::new(pool).await
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn get_users(&self) -> Result<Vec<UserGet>, super::RepositoryError> {
        unimplemented!()
    }

    async fn auth_user(
        &self,
        user_credentials: UserCredentials,
    ) -> Result<Option<UserIdentifiers>, RepositoryError> {
        let UserCredentials { email, password } = user_credentials;
        let Some((user_passswords_pair, user_identifiers)) =
            sqlx::query(&self.get_user_passwd_and_role_query)
                .bind(email)
                .map(
                    |row: PgRow| -> Result<(UserPasswordsPair, UserIdentifiers), RepositoryError> {
                        Ok((
                            UserPasswordsPair {
                                password_hashed: row.try_get("password_hashed")?,
                                password_salt: row.try_get("password_salt")?,
                            },
                            UserIdentifiers {
                                email: row.try_get("email")?,
                                id: row.try_get("user_id")?,
                                role: row.try_get("role")?,
                            },
                        ))
                    },
                )
                .fetch_optional(&self.pool)
                .await?
                .transpose()?
        else {
            return Ok(None);
        };

        let verified_password =
            verify(password, &user_passswords_pair.password_hashed).map_err(|err| {
                tracing::error!("Veryfing password error: {err}");
                RepositoryError::Encryption(err.to_string())
            })?;

        Ok(verified_password.then_some(user_identifiers))
    }

    async fn get_user_by_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<UserGet>, RepositoryError> {
        Ok(sqlx::query_as::<_, UserGet>(&self.get_user_by_id_query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?)
    }

    async fn create_user(&self, user_post: UserPost) -> Result<UserGet, RepositoryError> {
        let uuid = Uuid::new_v4();
        let salt = generate_random_salt();
        let password_hashed = hash_with_salt(user_post.credentials.password, DEFAULT_COST, salt)?;
        let now = Utc::now();

        let mut tx = self.pool.begin().await?;
        sqlx::query("INSERT INTO user_log_infos (user_id, email, password_hashed, password_salt) VALUES ($1, $2, $3, $4)")
            .bind(uuid)
            .bind(user_post.credentials.email.clone())
            .bind(password_hashed.to_string())
            .bind(salt)
            .execute(&mut *tx)
            .await?;

        sqlx::query("INSERT INTO user_accounts (user_id, phone_number, first_name, last_name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(uuid)
            .bind(user_post.phone_number.clone())
            .bind(user_post.account.first_name.clone())
            .bind(user_post.account.last_name.clone())
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await?;

        let created_user = UserGet {
            user_id: uuid,
            email: user_post.credentials.email,
            phone_number: user_post.phone_number,
            first_name: user_post.account.first_name,
            last_name: user_post.account.last_name,
            created_at: now,
            updated_at: now,
        };

        tx.commit().await?;

        Ok(created_user)
    }

    async fn update_user(
        &self,
        user_id: Uuid,
        user_put: UserAccountPut,
    ) -> Result<Option<UserGet>, RepositoryError> {
        let now = Utc::now();

        let mut tx = self.pool.begin().await?;
        let Some(mut user) = sqlx::query_as::<_, UserGet>(&self.get_user_by_id_query)
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?
        else {
            return Ok(None);
        };

        update_user_account(&mut user, user_put);
        user.updated_at = now;

        sqlx::query("UPDATE user_accounts SET first_name = $2, last_name = $3, updated_at = $4 WHERE user_id = $1")
            .bind(user.user_id)
            .bind(user.first_name.clone())
            .bind(user.last_name.clone())
            .bind(user.updated_at)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;

        Ok(Some(user))
    }

    async fn delete_user(&self, user_id: uuid::Uuid) -> Result<(), RepositoryError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(r#"DELETE FROM user_accounts where user_id = $1"#)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(r#"DELETE FROM user_log_infos where user_id = $1"#)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
