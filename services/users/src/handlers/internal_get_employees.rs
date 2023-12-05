use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;
use common_types::{User, UserRoles};
use uuid::Uuid;

use crate::middlewares::auth::Authorization;
use crate::repository::UserRepository;
use crate::types::users::UserGet;

use super::errors::PublicError;

pub async fn internal_get_employees(
    State(user_repository): State<Arc<dyn UserRepository>>,
) -> Result<Json<Vec<User>>, PublicError> {
    Ok(Json(
        user_repository
            .get_employees()
            .await?
            .into_iter()
            .map(|e| User {
                user_id: e.user_id,
                email: e.email,
                phone_number: e.phone_number,
                fist_name: e.first_name,
                last_name: e.last_name,
            })
            .collect(),
    ))
}
