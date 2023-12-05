use chrono::{DateTime, NaiveTime, Utc};
use common_types::User;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::types::PgTimeTz, FromRow};
use uuid::Uuid;
use validator::{ValidationError, ValidationErrors};

use super::add_field_error;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ScheduleSlot {
    pub employee_id: Uuid,
    pub slot_start_time: DateTime<Utc>,
    pub slot_end_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmployeeSlots {
    #[serde(flatten)]
    pub user: User,
    pub free_slots: Vec<Slots>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Slots {
    pub slot_start_time: DateTime<Utc>,
    pub slot_end_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ScheduleGet {
    pub schedule_id: Uuid,
    pub service_id: Uuid,
    pub employee_id: Uuid,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub time: ScheduleRange,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchedulePost {
    pub service_id: Uuid,
    pub employee_id: Uuid,
    #[serde(flatten)]
    pub time: ScheduleRange,
}

impl SchedulePost {
    pub fn custom_validate(&self) -> Result<(), ValidationErrors> {
        let mut result = Ok(());
        self.time.validate_time_range(&mut result);

        result
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ScheduleRange {
    pub start_shift: NaiveTime,
    pub end_shift: NaiveTime,
}

impl ScheduleRange {
    pub fn validate_time_range(&self, result: &mut Result<(), ValidationErrors>) {
        if self.start_shift > self.end_shift {
            let mut err = ValidationError::new("invalid_time_range");
            err.add_param("start_shift".into(), &self.start_shift);
            err.add_param("end_shift".into(), &self.end_shift);
            add_field_error(result, "time", err);
        }
    }
}
