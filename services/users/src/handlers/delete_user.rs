use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::middlewares::auth::RequireAuth;
use crate::repository::UserRepository;

use super::errors::PublicError;

pub async fn delete_user(
    RequireAuth(user_identifiers): RequireAuth,
    Path(user_id): Path<Uuid>,
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<impl IntoResponse, PublicError> {
    if user_identifiers.id != user_id {
        return Err(PublicError::Unauthorized);
    }
    Ok((
        StatusCode::NO_CONTENT,
        user_repository.delete_user(user_id).await?,
    ))
}
