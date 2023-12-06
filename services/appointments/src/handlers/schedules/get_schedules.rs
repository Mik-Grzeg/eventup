use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::{schedules::ScheduleGet},
};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{extract::State, Json};


pub async fn get_schedules(
    AuthorizationControl(_user_identifiers): AuthorizationControl,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
) -> Result<Json<Vec<ScheduleGet>>, PublicError> {
    Ok(Json(service_repository.get_schedules().await?))
}
