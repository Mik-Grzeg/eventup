use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIdentifiers {
    pub id: Uuid,
    pub email: String,
}
