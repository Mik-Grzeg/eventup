use std::sync::Arc;

use crate::repository::ServiceRepository;

use super::super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn delete_service(
    AuthExtractor(user_identifiers): AuthExtractor,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Path(service_id): Path<Uuid>,
) -> Result<StatusCode, PublicError> {
    unimplemented!()
}
