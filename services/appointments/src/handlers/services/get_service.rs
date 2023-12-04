use std::sync::Arc;

use crate::{repository::ServiceRepository, types::services::ServiceGet};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{extract::{State, Path}, Json};
use common_types::UserRoles;
use uuid::Uuid;

pub async fn get_service(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    Path(service_id): Path<Uuid>,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
) -> Result<Json<ServiceGet>, PublicError> {
    let service = service_repository
        .get_service_by_id(service_id)
        .await?
        .ok_or(PublicError::NotFound)?;
    match (service.active, user_identifiers) {
        (_, Some(identifiers)) if identifiers.role == UserRoles::Admin => {
            tracing::info!("services requested by admin");
            Ok(Json(service))
        }
        (false, _) => Err(PublicError::Unauthorized),
        (true, _) => {
            tracing::info!("services requested by regular");
            Ok(Json(service))
        }
    }
}
