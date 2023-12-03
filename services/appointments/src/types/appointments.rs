use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AppointmentGet {
    pub appointment_id: Uuid,
    pub service_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub client_id: Uuid,
    pub client_name: String,

    pub employee_id: Uuid,

    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,

    pub price_expected: f32,
    #[serde(default)]
    pub price_final: Option<f32>,
    #[serde(default)]
    pub discount: Option<f32>,

    #[serde(default)]
    pub canceled: bool,
    #[serde(default)]
    pub cancellation_reason: Option<String>,
    #[serde(default)]
    pub served: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AppointmentPost {
    pub service_id: Uuid,
    pub client_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub client_name: String,

    pub employee_id: Uuid,

    #[serde(flatten)]
    pub time: AppointmentTime,
}

impl AppointmentPost {
    pub fn custom_validate(&self) -> Result<(), ValidationErrors> {
        let mut result = self.validate();
        self.time.validate_time_range(&mut result);

        result
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AppointmentPut {
    #[validate(length(min = 1, max = 255))]
    pub client_name: Option<String>,

    pub employee_id: Option<Uuid>,
    #[serde(flatten)]
    pub time: Option<AppointmentTime>,
}

impl AppointmentPut {
    pub fn custom_validate(&self) -> Result<(), ValidationErrors> {
        let mut result = self.validate();
        if let Some(time) = self.time.as_ref() {
            time.validate_time_range(&mut result);
        }

        result
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentTime {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl AppointmentTime {
    pub fn validate_time_range(&self, result: &mut Result<(), ValidationErrors>) {
        if self.start_time > self.end_time {
            let mut err = ValidationError::new("invalid_time_range");
            err.add_param("start_time".into(), &self.start_time);
            err.add_param("end_time".into(), &self.end_time);
            add_field_error(result, "time", err);
        }
    }

    pub fn validate_with_duration(&self, duration: &Duration) -> Result<(), ValidationErrors> {
        let mut result = Ok(());
        if (self.start_time - self.end_time).num_seconds() % duration.num_seconds() != 0 {
            let mut err = ValidationError::new("invalid_time_range");
            err.add_param("start_time".into(), &self.start_time);
            err.add_param("end_time".into(), &self.end_time);
            err.add_param(
                "expected_service_time_in_seconds".into(),
                &duration.num_seconds(),
            );
            add_field_error(&mut result, "time", err);
        }
        result
    }
}

impl AppointmentPut {
    fn update_existing_appointment(self, existing_appointment: &mut AppointmentGet) {
        if let Some(client_name) = self.client_name {
            existing_appointment.client_name = client_name
        }

        if let Some(employee_id) = self.employee_id {
            existing_appointment.employee_id = employee_id
        }

        if let Some(time) = self.time {
            existing_appointment.start_time = time.start_time;
            existing_appointment.end_time = time.end_time;
        }
    }
}

fn add_field_error(
    errors: &mut Result<(), ValidationErrors>,
    field: &'static str,
    error: ValidationError,
) {
    if errors.is_ok() {
        *errors = Err(ValidationErrors::new());
    }
    errors.as_mut().unwrap_err().add(field, error);
}
