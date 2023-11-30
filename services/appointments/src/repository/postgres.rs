use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::config::AppConfig;
use crate::types::appointments::AppointmentGet;

use super::error::RepositoryError;
use super::AppointmentRepository;

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
pub struct PostgresRepo {
    pool: PgPool,
}

impl PostgresRepo {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
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

#[async_trait()]
impl AppointmentRepository for PostgresRepo {
    async fn get_user_appointments(
        &self,
        user_identifiers: &common_types::UserIdentifiers,
    ) -> Result<Vec<crate::types::appointments::AppointmentGet>, RepositoryError> {
        Ok(
            sqlx::query_as::<_, AppointmentGet>("SELECT * FROM appointments WHERE client_id = $1")
                .bind(user_identifiers.id)
                .fetch_all(&self.pool)
                .await?,
        )
    }

    async fn get_appointment_by_id(
        &self,
        user_identifiers: &common_types::UserIdentifiers,
        appointment_id: uuid::Uuid,
    ) -> Result<Option<crate::types::appointments::AppointmentGet>, RepositoryError> {
        unimplemented!()
    }

    async fn create_appointment(
        &self,
        user_identifiers: &common_types::UserIdentifiers,
        appointment: crate::types::appointments::AppointmentPost,
    ) -> Result<crate::types::appointments::AppointmentGet, RepositoryError> {
        unimplemented!()
    }

    async fn update_appointment(
        &self,
        user_identifiers: &common_types::UserIdentifiers,
        appointment_id: uuid::Uuid,
        appointment: crate::types::appointments::AppointmentPut,
    ) -> Result<Option<crate::types::appointments::AppointmentGet>, RepositoryError> {
        unimplemented!()
    }

    async fn delete_appointment(
        &self,
        user_identifiers: &common_types::UserIdentifiers,
        appointment_id: uuid::Uuid,
    ) -> Result<(), RepositoryError> {
        unimplemented!()
    }
}
