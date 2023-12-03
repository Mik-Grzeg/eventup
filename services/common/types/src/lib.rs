use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "user_roles", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRoles {
    Admin,
    Employee,
    Regular,
}

impl UserRoles {
    pub fn regular() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserIdentifiers {
    pub id: Uuid,
    pub email: String,
    pub role: UserRoles,
}
