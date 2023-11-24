use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    user_id: Uuid,
    email: String,
    password: String,
    phone_number: String,
    first_name: String,
    last_name: String,
}
