use std::sync::Arc;

use crate::{
    repository::AppointmentRepository,
    types::appointments::{AppointmentCancel},
};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

pub async fn cancel_appointment(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    Path(appointment_id): Path<Uuid>,
    State(appointment_repository): State<Arc<dyn AppointmentRepository>>,
    Json(appointment_cancel): Json<AppointmentCancel>,
) -> Result<Json<Option<()>>, PublicError> {
    tracing::info!("Requested by user {user_identifiers:?}");
    match user_identifiers {
        Some(identifiers) => Ok(Json(
            appointment_repository
                .cancel_appointment(appointment_id, appointment_cancel, &identifiers)
                .await?,
        )),
        _ => Err(PublicError::Unauthorized),
    }
}
