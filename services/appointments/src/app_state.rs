use auth_extractor::AuthClient;
use axum::extract::FromRef;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub access_control_client: AuthClient,
}

impl FromRef<AppState> for AuthClient {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.access_control_client.clone()
    }
}

impl AppState {
    pub async fn from_config(config: &AppConfig) -> Self {
        // let user_repository = Arc::new(PgUserRepository::from_config(config).await);
        let access_control_client = AuthClient::new(&config.access_control_url);
        Self {
            access_control_client,
        }
    }
}
