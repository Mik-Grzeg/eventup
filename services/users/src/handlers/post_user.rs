use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use validator::Validate;

use crate::repository::UserRepository;
use crate::types::users::{UserGet, UserPost};

use super::errors::PublicError;

pub async fn create_user(
    State(user_repository): State<Arc<dyn UserRepository>>,
    Json(user): Json<UserPost>,
) -> Result<(StatusCode, Json<UserGet>), PublicError> {
    user.validate()?;

    let user = user_repository.create_user(user).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
