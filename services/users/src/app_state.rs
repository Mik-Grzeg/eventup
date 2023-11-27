use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{
    config::AppConfig,
    repository::{postgres::PgUserRepository, UserRepository},
};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
}

impl FromRef<AppState> for Arc<dyn UserRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_repository.clone()
    }
}

impl AppState {
    pub async fn new(pool: PgPool) -> Self {
        let user_repository = Arc::new(PgUserRepository::new(pool).await);
        Self { user_repository }
    }
    pub async fn from(config: &AppConfig) -> Self {
        let user_repository = Arc::new(PgUserRepository::from_config(config).await);
        Self { user_repository }
    }
}
