use axum::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, StatusCode},
};
use common_types::UserIdentifiers;

use std::sync::Arc;

// Custom extractor for authentication
pub struct AuthExtractor(pub UserIdentifiers);

#[derive(Clone)]
pub struct AuthClient {
    client: reqwest::Client,
    url: Arc<String>,
}

impl AuthClient {
    pub fn new(url: &str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            url: Arc::new(url.into()),
        }
    }
}

impl std::ops::Deref for AuthClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthExtractor
where
    AuthClient: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        println!("MURVA JAZDA");
        let Some(auth_header) = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
        else {
            tracing::error!("Authorization header malformed or missing");
            return Err(StatusCode::FORBIDDEN);
        };

        let auth_client = AuthClient::from_ref(state);
        let response = auth_client
            .get(auth_client.url.as_str())
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .send()
            .await
            .map_err(|err| {
                tracing::error!("Access control api call failed: {err}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        match response.status() {
            reqwest::StatusCode::FORBIDDEN => return Err(StatusCode::FORBIDDEN),
            reqwest::StatusCode::UNAUTHORIZED => return Err(StatusCode::UNAUTHORIZED),
            status_code if !status_code.is_success() => {
                tracing::error!("No success status code returned from access control api: {status_code:?}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
            _ => {}
        }

        let user_identifiers = response.json::<UserIdentifiers>().await.map_err(|err| {
            tracing::error!("Unable to deserialize User Identifiers error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(AuthExtractor(user_identifiers))
    }
}
