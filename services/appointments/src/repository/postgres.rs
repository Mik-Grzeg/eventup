use async_trait::async_trait;
use chrono::{Duration, Utc};
use common_types::UserRoles;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::types::appointments::AppointmentGet;
use crate::types::services::{update_service, ServiceGet, ServicePost};

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
        user_id: &Uuid,
    ) -> Result<Vec<crate::types::appointments::AppointmentGet>, RepositoryError> {
        Ok(
            sqlx::query_as::<_, AppointmentGet>("SELECT * FROM appointments WHERE client_id = $1")
                .bind(user_id)
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
    ) -> Result<AppointmentGet, RepositoryError> {
        let uuid = Uuid::new_v4();
        let now = Utc::now();

        let (duration_in_sec, price, active) =
            self.get_service_metadata(appointment.service_id).await?;

        appointment
            .time
            .validate_with_duration(&Duration::seconds(duration_in_sec as i64))
            .map_err(|err| RepositoryError::ValidationError(err))?;


        let expected_price = price * ((appointment.time.start_time - appointment.time.end_time)
            .num_seconds() as f32
            / duration_in_sec as f32);

        sqlx::query(
            "INSERT INTO appointments (
                appointment_id,
                service_id,
                employee_id,
                client_id,
                client_name,
                start_time,
                end_time,
                price_expected,
                created_at,
                updated_at
            ) VALUES (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
)",
        )
        .bind(uuid)
        .bind(appointment.service_id)
        .bind(appointment.employee_id)
        .bind(appointment.client_id)
        .bind(appointment.client_name.clone())
        .bind(appointment.time.start_time)
        .bind(appointment.time.end_time)
        .bind(expected_price)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let appointment = AppointmentGet {
            appointment_id: uuid,
            service_id: appointment.service_id,
            created_at: now,
            updated_at: now,
            client_id: appointment.client_id,
            client_name: appointment.client_name,
            employee_id: appointment.employee_id,
            start_time: appointment.time.start_time,
            end_time: appointment.time.end_time,
            price_expected: expected_price,
            price_final: None,
            discount: None,
            canceled: false,
            cancellation_reason: None,
            served: false,
        };

        Ok(appointment)
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
    ) -> Result<Option<()>, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        let Some(user_id) = sqlx::query("SELECT client_id FROM WHERE appointment_id = $1")
            .bind(appointment_id)
            .map(|row: PgRow| -> Result<Uuid, RepositoryError> { Ok(row.try_get("client_id")?) })
            .fetch_optional(&mut *tx)
            .await?
            .transpose()?
        else {
            return Ok(None);
        };

        if !(user_id == user_identifiers.id || user_identifiers.role == UserRoles::Admin) {
            return Err(RepositoryError::Unauthorized);
        }

        sqlx::query("DELETE FROM appointments WHERE appointment_id = $1")
            .bind(appointment_id)
            .execute(&self.pool)
            .await?;

        tx.commit().await?;
        Ok(Some(()))
    }
}

#[async_trait]
impl ServiceRepository for PostgresRepo {
    async fn get_all_services(&self) -> Result<Vec<ServiceGet>, RepositoryError> {
        Ok(sqlx::query_as::<_, ServiceGet>("SELECT * FROM services")
            .fetch_all(&self.pool)
            .await?)
    }

    async fn get_active_services(&self) -> Result<Vec<ServiceGet>, RepositoryError> {
        Ok(
            sqlx::query_as::<_, ServiceGet>("SELECT * FROM services WHERE active = true")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    async fn get_service_metadata(
        &self,
        service_id: Uuid,
    ) -> Result<(i32, f32, bool), RepositoryError> {
        sqlx::query("SELECT duration_in_sec, price, active FROM services WHERE service_id = $1")
            .bind(service_id)
            .map(|row: PgRow| -> Result<(i32, f32, bool), RepositoryError> {
                Ok((
                    row.try_get("duration_in_sec")?,
                    row.try_get("price")?,
                    row.try_get("active")?,
                ))
            })
            .fetch_one(&self.pool)
            .await?
    }

    async fn create_service(&self, service: ServicePost) -> Result<ServiceGet, RepositoryError> {
        let uuid = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query("INSERT INTO services (service_id, name, description, duration_in_sec, price, active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(uuid)
        .bind(service.name.clone())
        .bind(service.description.clone())
        .bind(service.duration_in_sec)
        .bind(service.price)
        .bind(service.active)
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
            active: service.active,
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
        let now = Utc::now();

        let mut tx = self.pool.begin().await?;
        let Some(mut old_service) = sqlx::query_as::<_, ServiceGet>(
            "SELECT * FROM services WHERE service_id = $1 FOR UPDATE",
        )
        .bind(service_id)
        .fetch_optional(&mut *tx)
        .await?
        else {
            return Ok(None);
        };

        update_service(&mut old_service, service);
        old_service.updated_at = now;

        sqlx::query("UPDATE services SET name = $1, description = $2, duration_in_sec = $3, price = $4, active = $5 WHERE service_id = $6")
            .bind(old_service.name.clone())
            .bind(old_service.description.clone())
            .bind(old_service.duration_in_sec)
            .bind(old_service.price)
            .bind(old_service.active)
            .bind(service_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;

        Ok(Some(old_service))
    }
}
