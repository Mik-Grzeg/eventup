use uuid::Uuid;

use crate::types::users::User;
pub mod postgres;

pub struct RepositoryError; 

pub trait UserRepository {
    fn get_users(&self) -> Result<Vec<User>, RepositoryError>;
    fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, RepositoryError>;
    fn create_user(&self, user: &User) -> Result<(), RepositoryError>;
    fn update_user(&self, user: &User) -> Result<(), RepositoryError>;
    fn delete_user(&self, user_id: Uuid) -> Result<(), RepositoryError>;
}
