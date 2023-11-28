use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, StatusCode},
};
use chrono::{TimeZone, Utc};
use jsonwebtoken::TokenData;

use crate::{
    app_state::AppState,
    types::{
        jwt::{token_is_valid, JWTClaims, LoginTokenRespone},
        users::UserIdentifiers,
    },
};

pub struct RequireAuth(pub UserIdentifiers);

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(LoginTokenRespone::try_from)
            .transpose()
            .map_err(|err| {
                tracing::error!("Unable to convert auth header to Token, error: {err}");
                StatusCode::BAD_REQUEST
            })?;

        let app_state = AppState::from_ref(state);
        let secret = app_state.secret;

        let token = auth_header
            .and_then(|hdr| match token_is_valid(&hdr.token, &secret) {
                Ok(token) => Some(token),
                Err(err) => {
                    tracing::error!("Auth middleware error: {err}");
                    None
                }
            })
            .and_then(|token| {
                if token.claims.exp < Utc::now().timestamp() {
                    let expiration_time = Utc.timestamp_opt(token.claims.exp, 0);
                    tracing::error!(
                        "Token for user_id={} email={} has expired at {expiration_time:?}",
                        token.claims.sub,
                        token.claims.id
                    );
                    None
                } else {
                    Some(token)
                }
            })
            .ok_or(StatusCode::FORBIDDEN)?;

        let user_identifiers = UserIdentifiers {
            email: token.claims.sub,
            id: token.claims.id,
        };
        Ok(RequireAuth(user_identifiers))
    }
}
