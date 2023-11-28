use chrono::{DateTime, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserAccountPut {
    #[validate(length(min = 1))]
    pub first_name: Option<String>,
    #[validate(length(min = 1))]
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserPost {
    #[serde(flatten)]
    #[validate]
    pub credentials: UserCredentials,
    #[validate(phone)]
    pub phone_number: String,
    #[serde(flatten)]
    #[validate]
    pub account: UserAccountPut,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserGet {
    pub user_id: Uuid,
    pub email: String,
    pub phone_number: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserCredentials {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(FromRow)]
pub struct UserPasswordsPair {
    pub password_hashed: String,
    pub password_salt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIdentifiers {
    pub id: Uuid,
    pub email: String,
}

pub fn update_user_account(old_user_data: &mut UserGet, updated_user_data: UserAccountPut) {
    old_user_data.first_name = updated_user_data
        .first_name
        .or(old_user_data.first_name.take());
    old_user_data.last_name = updated_user_data
        .last_name
        .or(old_user_data.last_name.take());
}

pub fn generate_random_salt() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    bytes
}
