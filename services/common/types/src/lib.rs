use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone)]
#[sqlx(type_name = "user_roles", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRoles {
    Admin,
    Regular,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIdentifiers {
    pub id: Uuid,
    pub email: String,
    pub role: UserRoles,
}
