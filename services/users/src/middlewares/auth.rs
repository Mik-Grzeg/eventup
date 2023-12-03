use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, StatusCode},
};
use chrono::{TimeZone, Utc};

use crate::{
    app_state::AppState,
    types::jwt::{token_is_valid, LoginTokenRespone},
};

use common_types::UserIdentifiers;

pub struct Authorization(pub Option<UserIdentifiers>);

#[async_trait]
impl<S> FromRequestParts<S> for Authorization
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) else {
            return Ok(Authorization(None));
        };

        let auth_header = auth_header
            .to_str()
            .map_err(|err| {
                tracing::error!("Authorization header not convertible to &str: error = {err}");
                StatusCode::BAD_REQUEST
            })
            .and_then(|token| {
                LoginTokenRespone::try_from(token).map_err(|err| {
                    tracing::error!("Authorization header malformed: error = {err}");
                    StatusCode::BAD_REQUEST
                })
            })?;

        let app_state = AppState::from_ref(state);
        let secret = app_state.secret;

        let token = match token_is_valid(&auth_header.token, &secret) {
            Ok(token) if token.claims.exp < Utc::now().timestamp() => {
                let expiration_time = Utc.timestamp_opt(token.claims.exp, 0);
                tracing::error!(
                    "Token for user_id={} email={} has expired at {expiration_time:?}",
                    token.claims.sub,
                    token.claims.id
                );
                return Err(StatusCode::FORBIDDEN);
            }
            Ok(token) => token,
            Err(err) => {
                tracing::error!("Authorization header not convertible to &str: error = {err}");
                return Err(StatusCode::BAD_REQUEST);
            }
        };

        let user_identifiers = UserIdentifiers {
            email: token.claims.sub,
            id: token.claims.id,
            role: token.claims.role,
        };
        Ok(Authorization(Some(user_identifiers)))
    }
}
