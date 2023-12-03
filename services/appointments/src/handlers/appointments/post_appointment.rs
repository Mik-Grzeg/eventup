use std::sync::Arc;

use crate::{
    repository::AppointmentRepository,
    types::appointments::{AppointmentGet, AppointmentPost},
};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, State},
    Json,
};
use common_types::UserRoles;

pub async fn create_appointment(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    State(appointment_repository): State<Arc<dyn AppointmentRepository>>,
    Json(appointment): Json<AppointmentPost>,
) -> Result<Json<AppointmentGet>, PublicError> {
    tracing::info!("Requested by user {user_identifiers:?}");
    match user_identifiers {
        Some(identifiers)
            if (identifiers.role == UserRoles::Admin || identifiers.id == appointment.client_id) =>
        {
            let user_appointments = appointment_repository
                .create_appointment(&identifiers, appointment)
                .await?;

            Ok(Json(user_appointments))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
