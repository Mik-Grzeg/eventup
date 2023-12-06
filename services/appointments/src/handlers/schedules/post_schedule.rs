use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::{
        schedules::{ScheduleGet, SchedulePost},
    },
};


use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{extract::State, http::StatusCode, Json};
use common_types::UserRoles;

pub async fn post_schedule(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Json(schedule): Json<SchedulePost>,
) -> Result<(StatusCode, Json<ScheduleGet>), PublicError> {
    match user_identifiers {
        Some(identifiers)
            if identifiers.role == UserRoles::Admin
                || (identifiers.role == UserRoles::Employee
                    && identifiers.id == schedule.employee_id) =>
        {
            schedule.custom_validate()?;

            let schedule = service_repository.create_schedule(schedule).await?;
            Ok((StatusCode::CREATED, Json(schedule)))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
