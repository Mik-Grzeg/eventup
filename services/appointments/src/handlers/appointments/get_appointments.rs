use std::sync::Arc;

use crate::{repository::AppointmentRepository, types::appointments::AppointmentGet};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, State},
    Json,
};
use common_types::UserRoles;
use uuid::Uuid;

pub async fn get_appointments_for_user(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    Path(user_id): Path<Uuid>,
    State(appointment_repository): State<Arc<dyn AppointmentRepository>>,
) -> Result<Json<Vec<AppointmentGet>>, PublicError> {
    tracing::info!("Requested by user {user_identifiers:?}");
    match user_identifiers {
        Some(identifiers)
            if (identifiers.role == UserRoles::Admin || identifiers.id == user_id) =>
        {
            let user_appointments = appointment_repository
                .get_user_appointments(&user_id)
                .await?;

            Ok(Json(user_appointments))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
