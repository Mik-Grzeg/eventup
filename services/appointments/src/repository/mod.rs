use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common_types::UserIdentifiers;
use uuid::Uuid;

use error::RepositoryError;

use crate::types::{
    appointments::{AppointmentCancel, AppointmentGet, AppointmentPost, AppointmentPut},
    schedules::{ScheduleGet, SchedulePost, ScheduleSlot},
    services::{ServiceGet, ServicePost, ServicePut},
};

pub mod error;
pub mod postgres;

#[async_trait]
pub trait AppointmentRepository: Send + Sync {
    async fn get_user_appointments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<AppointmentGet>, RepositoryError>;
    async fn create_appointment(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment: AppointmentPost,
    ) -> Result<AppointmentGet, RepositoryError>;
    async fn update_appointment(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment_id: Uuid,
        appointment: AppointmentPut,
    ) -> Result<Option<AppointmentGet>, RepositoryError>;
    async fn delete_appointment(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment_id: Uuid,
    ) -> Result<Option<()>, RepositoryError>;
    async fn cancel_appointment(
        &self,
        appointment_id: uuid::Uuid,
        appointment_cancel: AppointmentCancel,
        user_identifiers: &UserIdentifiers,
    ) -> Result<Option<()>, RepositoryError>;
    async fn serve_appointment(
        &self,
        appointment_id: uuid::Uuid,
    ) -> Result<Option<()>, RepositoryError>;

    async fn get_free_slots_for_day(
        &self,
        datetime: DateTime<Utc>,
        service_id: Uuid,
    ) -> Result<Vec<ScheduleSlot>, RepositoryError>;
}

#[async_trait]
impl<AR: AppointmentRepository + ?Sized + 'static> AppointmentRepository for Arc<AR> {
    async fn get_user_appointments(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<AppointmentGet>, RepositoryError> {
        self.as_ref().get_user_appointments(user_id).await
    }

    async fn get_free_slots_for_day(
        &self,
        datetime: DateTime<Utc>,
        service_id: Uuid,
    ) -> Result<Vec<ScheduleSlot>, RepositoryError> {
        self.as_ref()
            .get_free_slots_for_day(datetime, service_id)
            .await
    }

    async fn create_appointment(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment: AppointmentPost,
    ) -> Result<AppointmentGet, RepositoryError> {
        self.as_ref()
            .create_appointment(user_identifiers, appointment)
            .await
    }

    async fn update_appointment(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment_id: Uuid,
        appointment: AppointmentPut,
    ) -> Result<Option<AppointmentGet>, RepositoryError> {
        self.as_ref()
            .update_appointment(user_identifiers, appointment_id, appointment)
            .await
    }

    async fn delete_appointment(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment_id: Uuid,
    ) -> Result<Option<()>, RepositoryError> {
        self.as_ref()
            .delete_appointment(user_identifiers, appointment_id)
            .await
    }

    async fn cancel_appointment(
        &self,
        appointment_id: uuid::Uuid,
        appointment_cancel: AppointmentCancel,
        user_identifiers: &UserIdentifiers,
    ) -> Result<Option<()>, RepositoryError> {
        self.as_ref()
            .cancel_appointment(appointment_id, appointment_cancel, user_identifiers)
            .await
    }

    async fn serve_appointment(
        &self,
        appointment_id: uuid::Uuid,
    ) -> Result<Option<()>, RepositoryError> {
        self.as_ref().serve_appointment(appointment_id).await
    }
}

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    // services
    async fn get_all_services(&self) -> Result<Vec<ServiceGet>, RepositoryError>;
    async fn get_active_services(&self) -> Result<Vec<ServiceGet>, RepositoryError>;
    async fn get_service_by_id(
        &self,
        service_id: Uuid,
    ) -> Result<Option<ServiceGet>, RepositoryError>;
    async fn get_service_metadata(
        &self,
        service_id: Uuid,
    ) -> Result<(i32, f32, bool), RepositoryError>;
    async fn create_service(&self, service: ServicePost) -> Result<ServiceGet, RepositoryError>;
    async fn update_service(
        &self,
        service: ServicePut,
        service_id: Uuid,
    ) -> Result<Option<ServiceGet>, RepositoryError>;
    // schedules
    async fn get_schedules(&self) -> Result<Vec<ScheduleGet>, RepositoryError>;
    async fn create_schedule(&self, schedule: SchedulePost)
        -> Result<ScheduleGet, RepositoryError>;
}

#[async_trait]
impl<SR: ServiceRepository + ?Sized + 'static> ServiceRepository for Arc<SR> {
    async fn get_all_services(&self) -> Result<Vec<ServiceGet>, RepositoryError> {
        self.as_ref().get_all_services().await
    }

    async fn get_service_by_id(
        &self,
        service_id: Uuid,
    ) -> Result<Option<ServiceGet>, RepositoryError> {
        self.as_ref().get_service_by_id(service_id).await
    }

    async fn get_service_metadata(
        &self,
        service_id: Uuid,
    ) -> Result<(i32, f32, bool), RepositoryError> {
        self.as_ref().get_service_metadata(service_id).await
    }

    async fn get_active_services(&self) -> Result<Vec<ServiceGet>, RepositoryError> {
        self.as_ref().get_active_services().await
    }
    async fn create_service(&self, service: ServicePost) -> Result<ServiceGet, RepositoryError> {
        self.as_ref().create_service(service).await
    }
    async fn update_service(
        &self,
        service: ServicePut,
        service_id: Uuid,
    ) -> Result<Option<ServiceGet>, RepositoryError> {
        self.as_ref().update_service(service, service_id).await
    }

    // schedules
    async fn get_schedules(&self) -> Result<Vec<ScheduleGet>, RepositoryError> {
        self.as_ref().get_schedules().await
    }

    async fn create_schedule(
        &self,
        schedule: SchedulePost,
    ) -> Result<ScheduleGet, RepositoryError> {
        self.as_ref().create_schedule(schedule).await
    }
}
