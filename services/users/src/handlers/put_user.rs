use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use common_types::UserRoles;
use uuid::Uuid;
use validator::Validate;

use crate::middlewares::auth::Authorization;
use crate::repository::UserRepository;
use crate::types::users::{UserAccountPut, UserGet};

use super::errors::PublicError;

pub async fn update_user(
    Authorization(user_identifiers): Authorization,
    State(user_repository): State<Arc<dyn UserRepository>>,
    Path(user_id): Path<Uuid>,
    Json(user): Json<UserAccountPut>,
) -> Result<(StatusCode, Json<UserGet>), PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.role == UserRoles::Admin || user_id == identifiers.id => {
            user.validate()?;

            let user = user_repository.update_user(user_id, user).await?;
            Ok((StatusCode::OK, Json(user.ok_or(PublicError::NotFound)?)))
        }
        _ => Err(PublicError::Unauthorized),
    }
}
