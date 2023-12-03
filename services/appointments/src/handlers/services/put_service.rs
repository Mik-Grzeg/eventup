use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::services::{ServiceGet, ServicePut},
};

use super::super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use common_types::UserRoles;
use uuid::Uuid;
use validator::Validate;

pub async fn put_service(
    AuthExtractor(user_identifiers): AuthExtractor,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Path(service_id): Path<Uuid>,
    Json(service): Json<ServicePut>,
) -> Result<(StatusCode, Json<Option<ServiceGet>>), PublicError> {
    if user_identifiers.role != UserRoles::Admin {
        return Err(PublicError::Unauthorized);
    }

    service.validate()?;

    let service = service_repository
        .update_service(service, service_id)
        .await?;
    Ok((StatusCode::CREATED, Json(service)))
}
