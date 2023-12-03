use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::services::{ServiceGet, ServicePost},
};
use validator::Validate;

use super::super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use common_types::UserRoles;

pub async fn post_service(
    AuthExtractor(user_identifiers): AuthExtractor,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Json(service): Json<ServicePost>,
) -> Result<(StatusCode, Json<Option<ServiceGet>>), PublicError> {
    if user_identifiers.role != UserRoles::Admin {
        return Ok((StatusCode::UNAUTHORIZED, Json(None)));
    }

    service.validate()?;

    let service = service_repository.create_service(service).await?;
    Ok((StatusCode::CREATED, Json(Some(service))))
}
