use std::sync::Arc;

use axum::extract::FromRef;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;

use crate::{
    config::AppConfig,
    repository::{postgres::PgUserRepository, UserRepository},
};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub secret: Arc<String>,
}

impl FromRef<AppState> for Arc<dyn UserRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_repository.clone()
    }
}

impl AppState {
    pub async fn new(pool: PgPool) -> Self {
        let user_repository = Arc::new(PgUserRepository::new(pool).await);
        let secret: Arc<String> = Arc::new(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect(),
        );
        Self {
            user_repository,
            secret,
        }
    }

    pub async fn from(config: &AppConfig) -> Self {
        let user_repository = Arc::new(PgUserRepository::from_config(config).await);
        let secret = Arc::new(config.secret_key.clone());

        Self {
            user_repository,
            secret,
        }
    }
}
