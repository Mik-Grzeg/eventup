use std::{str::FromStr, sync::Arc};

use crate::{repository::AppointmentRepository, types::schedules::ScheduleSlot};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{format::parse_and_remainder, Date, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub date: Option<NaiveDate>,
}

pub async fn get_free_slots_for_day(
    Query(params): Query<Params>,
    Path(service_id): Path<Uuid>,
    State(appointment_repository): State<Arc<dyn AppointmentRepository>>,
) -> Result<Json<Vec<ScheduleSlot>>, PublicError> {
    let now = Utc::now();

    if params.date.is_some_and(|date| date < now.date_naive()) {
        return Err(PublicError::BadRequest(
            "Free slots can be request only in the future.".into(),
        ));
    }

    let datetime = dbg!(params)
        .date
        .and_then(|date| (date != now.date_naive()).then_some(date))
        .map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc())
        .unwrap_or(now);

    Ok(Json(
        appointment_repository
            .get_free_slots_for_day(datetime, service_id)
            .await?,
    ))
}
