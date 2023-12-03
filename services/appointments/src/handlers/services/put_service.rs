use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::services::{ServiceGet, ServicePut},
};

use super::super::errors::PublicError;
use auth_extractor::AuthorizationControl;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use common_types::UserRoles;
use uuid::Uuid;
use validator::Validate;

pub async fn put_service(
    AuthorizationControl(user_identifiers): AuthorizationControl,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Path(service_id): Path<Uuid>,
    Json(service): Json<ServicePut>,
) -> Result<(StatusCode, Json<Option<ServiceGet>>), PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.role == UserRoles::Admin => {
            service.validate()?;

            let service = service_repository
                .update_service(service, service_id)
                .await?;
            Ok((StatusCode::OK, Json(service)))
        }
        _ => return Err(PublicError::Unauthorized),
    }
}
