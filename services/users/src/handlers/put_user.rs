use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use uuid::Uuid;
use validator::Validate;

use crate::middlewares::auth::RequireAuth;
use crate::repository::UserRepository;
use crate::types::users::{UserAccountPut, UserGet};

use super::errors::PublicError;

pub async fn update_user(
    RequireAuth(user_identifiers): RequireAuth,
    State(user_repository): State<Arc<dyn UserRepository>>,
    Path(user_id): Path<Uuid>,
    Json(user): Json<UserAccountPut>,
) -> Result<(StatusCode, Json<UserGet>), PublicError> {
    if user_identifiers.id != user_id {
        return Err(PublicError::Unauthorized);
    }
    user.validate()?;

    let user = user_repository.update_user(user_id, user).await?;
    Ok((StatusCode::OK, Json(user.ok_or(PublicError::NotFound)?)))
}
