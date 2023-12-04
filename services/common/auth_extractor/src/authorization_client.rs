use axum::{async_trait, http::StatusCode};
use common_types::UserIdentifiers;
use std::sync::Arc;

use crate::Authorizable;

#[derive(Clone, Debug)]
pub struct AuthorizationClient {
    client: reqwest::Client,
    url: Arc<String>,
}

impl AuthorizationClient {
    pub fn new(url: &str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            url: Arc::new(url.into()),
        }
    }
}

impl std::ops::Deref for AuthorizationClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[async_trait]
impl Authorizable for AuthorizationClient {
    async fn authorize(&self, auth_header: &str) -> Result<Option<UserIdentifiers>, StatusCode> {
        let response = self
            .get(self.url.as_str())
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
                tracing::error!(
                    "No success status code returned from access control api: {status_code:?}"
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            _ => {}
        }

        let user_identifiers = response.json::<UserIdentifiers>().await.map_err(|err| {
            tracing::error!("Unable to deserialize User Identifiers error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Ok(Some(user_identifiers))
    }
}
