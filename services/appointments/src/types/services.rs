use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::types::PgInterval, FromRow};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ServiceGet {
    pub service_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "duration_in_sec")]
    pub duration: i64,
    pub price: f64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ServicePost {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 55555))]
    pub description: Option<String>,
    #[serde(rename = "duration_in_sec")]
    pub duration: i64,
    pub price: f64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ServicePut {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 55555))]
    pub description: Option<String>,
    #[serde(rename = "duration_in_sec")]
    pub duration: Option<i64>,
    pub price: Option<f64>,
}
