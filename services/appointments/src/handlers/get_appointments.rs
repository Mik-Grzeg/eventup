use std::sync::Arc;

use crate::{repository::AppointmentRepository, types::appointments::AppointmentGet};

use super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{extract::State, Json};

pub async fn get_appointments_for_user(
    AuthExtractor(user_identifiers): AuthExtractor,
    State(appointment_repository): State<Arc<dyn AppointmentRepository>>,
) -> Result<Json<Vec<AppointmentGet>>, PublicError> {
    tracing::info!("Request for user {user_identifiers:?}");
    let user_appointments = appointment_repository
        .get_user_appointments(&user_identifiers)
        .await?;

    Ok(Json(user_appointments))
}
