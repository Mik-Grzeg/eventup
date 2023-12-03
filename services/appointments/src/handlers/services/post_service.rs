use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::services::{ServiceGet, ServicePost},
};
use validator::Validate;

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use common_types::UserRoles;

pub async fn post_service(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Json(service): Json<ServicePost>,
) -> Result<(StatusCode, Json<ServiceGet>), PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.role == UserRoles::Admin => {
            service.validate()?;

            let service = service_repository.create_service(service).await?;
            Ok((StatusCode::CREATED, Json(service)))
        }
        _ => return Err(PublicError::Unauthorized),
    }
}
