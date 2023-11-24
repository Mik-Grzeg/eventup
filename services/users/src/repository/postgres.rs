use crate::config::AppConfig;
use crate::repository::UserRepository;

use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub async fn new(config: &AppConfig) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(config.pg_max_conn)
            .connect(&config.pg_url)
            .await
            .expect("Failed to create PostgreSQL pool.");
        tracing::info!("Initiated PosgreSQLG pool");

        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    fn get_users(&self) -> Result<Vec<crate::types::users::User>, super::RepositoryError> {
        unimplemented!()
    }

    fn get_user_by_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<crate::types::users::User>, super::RepositoryError> {
        unimplemented!()
    }

    fn create_user(&self, user: &crate::types::users::User) -> Result<(), super::RepositoryError> {
        unimplemented!()
    }

    fn update_user(&self, user: &crate::types::users::User) -> Result<(), super::RepositoryError> {
        unimplemented!()
    }

    fn delete_user(&self, user_id: uuid::Uuid) -> Result<(), super::RepositoryError> {
        unimplemented!()
    }
}
