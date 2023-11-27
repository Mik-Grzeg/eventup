use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use uuid::Uuid;
use validator::Validate;

use crate::repository::UserRepository;
use crate::types::users::UserAccountPut;

use super::errors::PublicError;

pub async fn update_user(
    State(user_repository): State<Arc<dyn UserRepository>>,
    Path(user_id): Path<Uuid>,
    Json(user): Json<UserAccountPut>,
) -> Result<impl IntoResponse, PublicError> {
    user.validate()?;

    let user = user_repository.update_user(user_id, user).await?;
    Ok((StatusCode::OK, Json(user)))
}
