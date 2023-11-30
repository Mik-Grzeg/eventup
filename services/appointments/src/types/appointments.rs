use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AppointmentGet {
    appointment_id: Uuid,
    service_id: Uuid,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    client_id: Uuid,
    client_name: String,

    employee_id: Uuid,
    
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,

    price_expected: f32,
    price_final: f32,
    discount: Option<f32>,

    canceled: bool,
    cancellation_reason: Option<String>,
    served: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentPost {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentPut {

}
