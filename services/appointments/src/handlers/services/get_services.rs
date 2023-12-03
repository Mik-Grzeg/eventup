use std::sync::Arc;

use crate::{repository::ServiceRepository, types::services::ServiceGet};

use super::super::errors::PublicError;
use auth_extractor::AuthExtractor;
use axum::{debug_handler, extract::State, Json};

pub async fn get_services(
    State(service_repository): State<Arc<dyn ServiceRepository>>,
) -> Result<Json<Vec<ServiceGet>>, PublicError> {
    Ok(Json(service_repository.get_services().await?))
}
