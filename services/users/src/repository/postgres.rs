use crate::config::AppConfig;
use crate::repository::{error::RepositoryError, UserRepository};
use crate::types::users::{generate_random_salt, update_user_account};
use crate::types::users::{UserAccountPut, UserGet, UserPost};
use chrono::Utc;

use async_trait::async_trait;
use bcrypt::{hash_with_salt, DEFAULT_COST};
use sqlx::postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("SQLX error: {error}");
        RepositoryError
    }
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    pool: PgPool,

    // queries
    get_user_by_id_query: String,
    _get_user_passwd_query: String,
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

        let _get_user_passwd_query =
            r#"SELECT password_hashed, password_salt WHERE user_id = $1"#.to_owned();

        Self {
            pool,
            get_user_by_id_query,
            _get_user_passwd_query,
        }
    }

    pub async fn from_config(config: &AppConfig) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(config.pg_max_conn)
            .connect(&config.pg_url)
            .await
            .expect("Failed to create PostgreSQL pool.");
        tracing::info!("Initiated PosgreSQLG pool");
        Self::new(pool).await
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn get_users(&self) -> Result<Vec<UserGet>, super::RepositoryError> {
        unimplemented!()
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
        let password_hashed = hash_with_salt(user_post.password, DEFAULT_COST, salt)?;
        let now = Utc::now();

        let mut tx = self.pool.begin().await?;
        sqlx::query("INSERT INTO user_log_infos (user_id, email, password_hashed, password_salt) VALUES ($1, $2, $3, $4)")
            .bind(uuid)
            .bind(user_post.email.clone())
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
            email: user_post.email,
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
