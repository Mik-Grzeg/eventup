use std::sync::Arc;

use auth_extractor::authorization_client::AuthorizationClient;
use axum::extract::FromRef;
use reqwest::Client;
use sqlx::PgPool;

use crate::{
    config::AppConfig,
    external_client::ExternalClient,
    repository::{postgres::PostgresRepo, AppointmentRepository, ServiceRepository},
};
use auth_extractor::Authorizable;

#[derive(Clone)]
pub struct AppState {
    pub access_control_client: Arc<dyn Authorizable>,
    pub service_repository: Arc<dyn ServiceRepository>,
    pub appointment_repository: Arc<dyn AppointmentRepository>,
    pub request_client: ExternalClient,
}

impl FromRef<AppState> for Arc<dyn Authorizable> {
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

impl FromRef<AppState> for ExternalClient {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.request_client.clone()
    }
}

impl AppState {
    pub async fn from_config(config: &AppConfig) -> Self {
        let postgres_repo = Arc::new(PostgresRepo::from_config(config).await);
        let appointment_repository = postgres_repo.clone();
        let service_repository = postgres_repo;
        let access_control_client = Arc::new(AuthorizationClient::new(&config.access_control_url));
        let request_client = ExternalClient::new(&config.employees_url);
        Self {
            access_control_client,
            appointment_repository,
            service_repository,
            request_client,
        }
    }

    pub async fn new(pool: PgPool) -> Self {
        let repo = Arc::new(PostgresRepo::new(pool).await);
        let appointment_repository = repo.clone();
        let service_repository = repo;
        let default_config = AppConfig::default();
        let access_control_client =
            Arc::new(AuthorizationClient::new(&default_config.access_control_url));

        let request_client = ExternalClient::new(&default_config.employees_url);

        Self {
            appointment_repository,
            service_repository,
            access_control_client,
            request_client,
        }
    }

    pub fn with_access_control(mut self, access_control: Arc<dyn Authorizable>) -> Self {
        self.access_control_client = access_control;
        self
    }
}
