use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;
use common_types::UserRoles;
use uuid::Uuid;

use crate::middlewares::auth::Authorization;
use crate::repository::UserRepository;
use crate::types::users::UserGet;

use super::errors::PublicError;

pub async fn get_employees(
    Authorization(user_identifiers): Authorization,
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<Json<Vec<UserGet>>, PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.role == UserRoles::Admin => {
            Ok(Json(user_repository.get_employees().await?))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
