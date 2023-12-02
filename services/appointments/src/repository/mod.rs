use std::sync::Arc;

use async_trait::async_trait;
use common_types::UserIdentifiers;
use uuid::Uuid;

use error::RepositoryError;

use crate::types::{appointments::{AppointmentGet, AppointmentPost, AppointmentPut}, services::{ServicePut, ServiceGet, ServicePost}};

pub mod error;
pub mod postgres;

#[async_trait]
pub trait AppointmentRepository: Send + Sync {
    async fn get_user_appointments(
        &self,
        user_identifiers: &UserIdentifiers,
    ) -> Result<Vec<AppointmentGet>, RepositoryError>;
    async fn get_appointment_by_id(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment_id: Uuid,
    ) -> Result<Option<AppointmentGet>, RepositoryError>;
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
    ) -> Result<(), RepositoryError>;
}

#[async_trait]
impl<AR: AppointmentRepository + ?Sized + 'static> AppointmentRepository for Arc<AR> {
    async fn get_user_appointments(
        &self,
        user_identifiers: &UserIdentifiers,
    ) -> Result<Vec<AppointmentGet>, RepositoryError> {
        self.as_ref().get_user_appointments(user_identifiers).await
    }

    async fn get_appointment_by_id(
        &self,
        user_identifiers: &UserIdentifiers,
        appointment_id: Uuid,
    ) -> Result<Option<AppointmentGet>, RepositoryError> {
        self.as_ref()
            .get_appointment_by_id(user_identifiers, appointment_id)
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
    ) -> Result<(), RepositoryError> {
        self.as_ref()
            .delete_appointment(user_identifiers, appointment_id)
            .await
    }
}

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn get_services(&self) -> Result<Vec<ServiceGet>, RepositoryError>;
    async fn create_service(&self, service: ServicePost) -> Result<ServiceGet, RepositoryError>;
    async fn update_service(
        &self,
        service: ServicePut,
        service_id: Uuid
    ) -> Result<Option<ServiceGet>, RepositoryError>;
    async fn delete_service(&self, service_id: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait]
impl<SR: ServiceRepository + ?Sized + 'static> ServiceRepository for Arc<SR> {
    async fn get_services(&self) -> Result<Vec<ServiceGet>, RepositoryError> {
        self.as_ref().get_services().await
    }
    async fn create_service(&self, service: ServicePost) -> Result<ServiceGet, RepositoryError> {
        self.as_ref().create_service(service).await
    }
    async fn update_service(
        &self,
        service: ServicePut,
        service_id: Uuid
    ) -> Result<Option<ServiceGet>, RepositoryError> {
        self.as_ref().update_service(service, service_id).await
    }
    async fn delete_service(&self, service_id: Uuid) -> Result<(), RepositoryError> {
        self.as_ref().delete_service(service_id).await
    }
}
