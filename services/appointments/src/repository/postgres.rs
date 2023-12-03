use async_trait::async_trait;
use chrono::Utc;
use sqlx::postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::types::appointments::AppointmentGet;
use crate::types::services::{ServiceGet, ServicePost};

use super::error::RepositoryError;
use super::{AppointmentRepository, ServiceRepository};

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

#[async_trait]
impl ServiceRepository for PostgresRepo {
    async fn get_services(&self) -> Result<Vec<ServiceGet>, RepositoryError> {
        Ok(sqlx::query_as::<_, ServiceGet>("SELECT * FROM services")
            .fetch_all(&self.pool)
            .await?)
    }

    async fn create_service(&self, service: ServicePost) -> Result<ServiceGet, RepositoryError> {
        let uuid = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query("INSERT INTO services (service_id, name, description, duration_in_sec, price, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(uuid)
        .bind(service.name.clone())
        .bind(service.description.clone())
        .bind(service.duration_in_sec)
        .bind(service.price)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let service_get = ServiceGet {
            service_id: uuid,
            name: service.name,
            description: service.description,
            duration_in_sec: service.duration_in_sec,
            price: service.price,
            updated_at: now,
            created_at: now,
        };

        Ok(service_get)
    }

    async fn update_service(
        &self,
        service: crate::types::services::ServicePut,
        service_id: uuid::Uuid,
    ) -> Result<Option<ServiceGet>, RepositoryError> {
        unimplemented!()
    }

    async fn delete_service(&self, service_id: uuid::Uuid) -> Result<(), RepositoryError> {
        unimplemented!()
    }
}
