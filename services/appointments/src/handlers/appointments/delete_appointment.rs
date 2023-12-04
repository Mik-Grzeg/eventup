use std::sync::Arc;

use crate::{
    repository::AppointmentRepository,
};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

pub async fn delete_appointment(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    Path(appointment_id): Path<Uuid>,
    State(appointment_repository): State<Arc<dyn AppointmentRepository>>,
) -> Result<Json<Option<()>>, PublicError> {
    tracing::info!("Requested by user {user_identifiers:?}");
    match user_identifiers {
        Some(identifiers) => {
            let user_appointments = appointment_repository
                .delete_appointment(&identifiers, appointment_id)
                .await?;

            Ok(Json(user_appointments))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
