use std::sync::Arc;

use crate::{
    repository::ServiceRepository,
    types::services::{ServiceGet, ServicePost},
};

use super::super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{debug_handler, extract::State, http::StatusCode, Json};

pub async fn post_service(
    AuthExtractor(user_identifiers): AuthExtractor,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
    Json(service): Json<ServicePost>,
) -> Result<(StatusCode, Json<ServiceGet>), PublicError> {
    unimplemented!()
}
