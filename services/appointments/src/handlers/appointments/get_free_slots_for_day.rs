use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
    sync::Arc,
};

use crate::{
    external_client::ExternalClient,
    repository::AppointmentRepository,
    types::schedules::{EmployeeSlots, ScheduleSlot, Slots},
};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{format::parse_and_remainder, Date, NaiveDate, Utc};
use common_types::User;
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
    State(request_client): State<ExternalClient>,
) -> Result<Json<Vec<EmployeeSlots>>, PublicError> {
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

    let mut slots = convert_slots_to_map(dbg!(
        appointment_repository
            .get_free_slots_for_day(datetime, service_id)
            .await?,
    ));
    tracing::info!("Queried slots: {slots:?}");
    

    let employees = dbg!(request_client.get::<Vec<User>>().await?);
    tracing::info!("Fetched employees: {employees:?}");

    let employees_slots = employees
        .into_iter()
        .map(|user| {
            slots.remove(&user.user_id).map(|slots| EmployeeSlots {
                user,
                free_slots: slots,
            })
        })
        .flatten()
        .collect();

    Ok(Json(employees_slots))
}

fn convert_slots_to_map(schedule_slots: Vec<ScheduleSlot>) -> HashMap<Uuid, Vec<Slots>> {
    let mut grouped_slots: HashMap<Uuid, Vec<Slots>> = HashMap::new();

    for slot in schedule_slots {
        grouped_slots
            .entry(slot.employee_id)
            .or_insert_with(Vec::new)
            .push(Slots {
                slot_start_time: slot.slot_start_time,
                slot_end_time: slot.slot_end_time,
            });
    }

    grouped_slots
}
