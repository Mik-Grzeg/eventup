use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;
use uuid::Uuid;

// use crate::app_state::UserRepo;
use crate::repository::UserRepository;
use crate::types::users::UserGet;

use super::errors::PublicError;

pub async fn get_user(
    Path(user_id): Path<Uuid>,
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<Json<Option<UserGet>>, PublicError> {
    Ok(Json(user_repository.get_user_by_id(user_id).await?))
}
