use axum::debug_handler;

use axum::{extract::State, Json};

use crate::app_state::AppState;
use crate::{
    repository::UserRepository,
    types::{
        jwt::{generate_jwt_token, LoginTokenRespone, TokenType},
        users::UserCredentials,
    },
};
use validator::Validate;

use super::errors::PublicError;

#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<UserCredentials>,
) -> Result<Json<LoginTokenRespone>, PublicError> {
    credentials.validate()?;
    let (user_repository, secret) = (state.user_repository, state.secret);
    user_repository
        .auth_user(credentials)
        .await?
        .map(|user_identifiers| {
            let token = generate_jwt_token(&user_identifiers, secret.as_ref());
            Json(LoginTokenRespone {
                token,
                r#type: TokenType::Bearer,
            })
        })
        .ok_or(PublicError::Unauthenticated)
}
