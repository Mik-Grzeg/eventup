use std::sync::Arc;

use crate::{repository::ServiceRepository, types::services::ServiceGet};

use super::super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{extract::State, Json, debug_handler};

pub async fn get_services(
    AuthExtractor(user_identifiers): AuthExtractor,
    State(service_repository): State<Arc<dyn ServiceRepository>>,
) -> Result<Json<Vec<ServiceGet>>, PublicError> {
    Ok(Json(service_repository.get_services().await?))
}
