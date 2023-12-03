use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use common_types::UserRoles;
use uuid::Uuid;

use crate::middlewares::auth::Authorization;
use crate::repository::UserRepository;

use super::errors::PublicError;

pub async fn delete_user(
    Authorization(user_identifiers): Authorization,
    Path(user_id): Path<Uuid>,
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<impl IntoResponse, PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.id == user_id || identifiers.role == UserRoles::Admin => {
            Ok((
                StatusCode::NO_CONTENT,
                user_repository.delete_user(user_id).await?,
            ))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
