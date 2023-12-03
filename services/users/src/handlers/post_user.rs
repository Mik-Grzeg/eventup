use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use common_types::UserRoles;
use validator::Validate;

use crate::middlewares::auth::Authorization;
use crate::repository::UserRepository;
use crate::types::users::{UserGet, UserPost};

use super::errors::PublicError;

pub async fn create_user(
    Authorization(user_identifiers): Authorization,
    State(user_repository): State<Arc<dyn UserRepository>>,
    Json(user): Json<UserPost>,
) -> Result<(StatusCode, Json<UserGet>), PublicError> {
    match user_identifiers {
        Some(identifiers) if identifiers.role == UserRoles::Admin => {
            user.validate()?;

            let user = user_repository.create_user(user).await?;
            Ok((StatusCode::CREATED, Json(user)))
        }
        _ => {
            if user.role != Some(UserRoles::Regular) && user.role.is_some() {
                return Err(PublicError::Unauthorized);
            };

            user.validate()?;

            let user = user_repository.create_user(user).await?;
            Ok((StatusCode::CREATED, Json(user)))
        }
    }
}
