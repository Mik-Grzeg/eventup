use std::sync::Arc;

use crate::{repository::ServiceRepository, types::services::ServiceGet};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{debug_handler, extract::State, Json};
use common_types::UserRoles;

pub async fn get_services(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
) -> Result<Json<Vec<ServiceGet>>, PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.role == UserRoles::Admin => {
            Ok(Json(service_repository.get_all_services().await?))
        }
        _ => Ok(Json(service_repository.get_active_services().await?)),
    }
}
