use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::services::{ServiceGet, ServicePut},
};

use super::super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{extract::{State, Path}, http::StatusCode, Json};
use uuid::Uuid;

pub async fn put_service(
    AuthExtractor(user_identifiers): AuthExtractor,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Path(service_id): Path<Uuid>,
    Json(service): Json<ServicePut>,
) -> Result<(StatusCode, Json<ServiceGet>), PublicError> {
    unimplemented!()
}
