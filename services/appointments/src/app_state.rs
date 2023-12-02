use std::sync::Arc;

use auth_extractor::AuthClient;
use axum::extract::FromRef;

use crate::{
    config::AppConfig,
    repository::{postgres::PostgresRepo, AppointmentRepository, ServiceRepository},
};

#[derive(Clone)]
pub struct AppState {
    pub access_control_client: AuthClient,
    pub service_repository: Arc<dyn ServiceRepository>,
    pub appointment_repository: Arc<dyn AppointmentRepository>,
}

impl FromRef<AppState> for AuthClient {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.access_control_client.clone()
    }
}

impl FromRef<AppState> for Arc<dyn AppointmentRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.appointment_repository.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ServiceRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.service_repository.clone()
    }
}

impl AppState {
    pub async fn from_config(config: &AppConfig) -> Self {
        let postgres_repo = Arc::new(PostgresRepo::from_config(config).await);
        let appointment_repository = postgres_repo.clone();
        let service_repository = postgres_repo;
        let access_control_client = AuthClient::new(&config.access_control_url);
        Self {
            access_control_client,
            appointment_repository,
            service_repository,
        }
    }
}
