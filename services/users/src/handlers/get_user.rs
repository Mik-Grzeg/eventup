use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;
use uuid::Uuid;

use crate::middlewares::auth::RequireAuth;
// use crate::app_state::UserRepo;
use crate::repository::UserRepository;
use crate::types::users::{UserGet, UserIdentifiers};

use super::errors::PublicError;

pub async fn get_user(
    RequireAuth(user_identifiers): RequireAuth,
    Path(user_id): Path<Uuid>,
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<Json<UserGet>, PublicError> {
    if user_identifiers.id != user_id {
        return Err(PublicError::Unauthorized);
    }
    Ok(Json(
        user_repository
            .get_user_by_id(user_id)
            .await?
            .ok_or(PublicError::NotFound)?,
    ))
}
