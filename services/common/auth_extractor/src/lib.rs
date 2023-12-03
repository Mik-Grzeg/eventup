use axum::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, StatusCode},
};
use common_types::UserIdentifiers;

use std::convert::Infallible;
use std::sync::Arc;

pub mod authorization_client;

#[cfg(feature = "test-utils")]
pub mod mock;

#[async_trait]
pub trait Authorizable: std::fmt::Debug + Send + Sync {
    async fn authorize(&self, auth_header: &str) -> Result<Option<UserIdentifiers>, StatusCode>;
}

// Custom extractor for authentication
pub struct AuthorizationControl(pub Option<UserIdentifiers>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthorizationControl
where
    Arc<dyn Authorizable>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) else {
            return Ok(AuthorizationControl(None));
        };

        let auth_header = auth_header.to_str().map_err(|err| {
            tracing::error!("Authorization header malformed or missing: error = {err}");
            StatusCode::BAD_REQUEST
        })?;

        let user_identifiers = Arc::<dyn Authorizable>::from_ref(state)
            .authorize(auth_header)
            .await?;

        tracing::error!("PLZ HELP ME KEK");
        Ok(AuthorizationControl(user_identifiers))
    }
}
