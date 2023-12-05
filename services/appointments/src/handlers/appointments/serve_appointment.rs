use std::sync::Arc;

use crate::{
    repository::AppointmentRepository,
    types::appointments::{AppointmentCancel, AppointmentGet, AppointmentPost},
};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, State},
    Json,
};
use common_types::UserRoles;
use uuid::Uuid;

pub async fn serve_appointment(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    Path(appointment_id): Path<Uuid>,
    State(appointment_repository): State<Arc<dyn AppointmentRepository>>,
) -> Result<Json<Option<()>>, PublicError> {
    tracing::info!("Requested by user {user_identifiers:?}");
    match user_identifiers {
        Some(identifiers)
            if (identifiers.role == UserRoles::Admin
                || identifiers.role == UserRoles::Employee) =>
        {
            Ok(Json(
                appointment_repository
                    .serve_appointment(appointment_id)
                    .await?,
            ))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
