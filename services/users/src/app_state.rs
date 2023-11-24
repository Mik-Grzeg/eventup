use crate::{
    config::AppConfig,
    repository::{postgres::PgUserRepository, UserRepository},
};

pub struct AppState {
    user_repository: Box<dyn UserRepository>,
}

impl AppState {
    pub async fn from(config: &AppConfig) -> Self {
        let user_repository = Box::new(PgUserRepository::new(config).await);

        Self { user_repository }
    }
}
