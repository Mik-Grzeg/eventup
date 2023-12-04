use std::sync::Arc;

use crate::{repository::ServiceRepository, types::{services::ServiceGet, schedules::ScheduleGet}};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{extract::State, Json};
use common_types::UserRoles;

pub async fn get_schedules(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
) -> Result<Json<Vec<ScheduleGet>>, PublicError> {
    Ok(Json(service_repository.get_schedules().await?))
}
