use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ServiceGet {
    pub service_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub duration_in_sec: i32,
    pub price: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ServicePost {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 55555))]
    pub description: Option<String>,
    pub duration_in_sec: i32,
    pub price: f32,
    #[serde(default = "default_as_true")]
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ServicePut {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 55555))]
    pub description: Option<String>,
    pub duration_in_sec: Option<i32>,
    pub price: Option<f32>,
    pub active: Option<bool>,
}

pub fn update_service(old_service_data: &mut ServiceGet, updated_service_data: ServicePut) {
    if let Some(name) = updated_service_data.name {
        old_service_data.name = name
    }

    if updated_service_data.description.is_some() {
        old_service_data.description = updated_service_data.description
    }

    if let Some(duration_is_sec) = updated_service_data.duration_in_sec {
        old_service_data.duration_in_sec = duration_is_sec
    }

    if let Some(price) = updated_service_data.price {
        old_service_data.price = price
    }

    if let Some(active) = updated_service_data.active {
        old_service_data.active = active
    }
}

fn default_as_true() -> bool {
    true
}
