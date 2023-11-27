use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::repository::UserRepository;

use super::errors::PublicError;

pub async fn delete_user(
    Path(user_id): Path<Uuid>,
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<impl IntoResponse, PublicError> {
    Ok((StatusCode::NO_CONTENT, user_repository.delete_user(user_id).await?))
}
