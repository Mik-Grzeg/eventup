use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;
use common_types::UserRoles;
use uuid::Uuid;

use crate::middlewares::auth::Authorization;
use crate::repository::UserRepository;
use crate::types::users::UserGet;

use super::errors::PublicError;

pub async fn get_user(
    Authorization(user_identifiers): Authorization,
    Path(user_id): Path<Uuid>,
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<Json<UserGet>, PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.id == user_id || identifiers.role == UserRoles::Admin => {
            Ok(Json(
                user_repository
                    .get_user_by_id(user_id)
                    .await?
                    .ok_or(PublicError::NotFound)?,
            ))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
