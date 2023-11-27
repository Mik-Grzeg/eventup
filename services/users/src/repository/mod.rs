use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::types::users::{UserAccountPut, UserGet, UserPost};
use error::RepositoryError;

pub mod error;
pub mod postgres;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self) -> Result<Vec<UserGet>, RepositoryError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserGet>, RepositoryError>;
    async fn create_user(&self, user: UserPost) -> Result<UserGet, RepositoryError>;
    async fn update_user(
        &self,
        user_id: Uuid,
        user: UserAccountPut,
    ) -> Result<Option<UserGet>, RepositoryError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait]
impl<UR: UserRepository + ?Sized + 'static> UserRepository for Arc<UR> {
    async fn get_users(&self) -> Result<Vec<UserGet>, RepositoryError> {
        self.as_ref().get_users().await
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserGet>, RepositoryError> {
        self.as_ref().get_user_by_id(user_id).await
    }

    async fn create_user(&self, user: UserPost) -> Result<UserGet, RepositoryError> {
        self.as_ref().create_user(user).await
    }

    async fn update_user(
        &self,
        user_id: Uuid,
        user: UserAccountPut,
    ) -> Result<Option<UserGet>, RepositoryError> {
        self.as_ref().update_user(user_id, user).await
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), RepositoryError> {
        self.as_ref().delete_user(user_id).await
    }
}
