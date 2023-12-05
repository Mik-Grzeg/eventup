use std::sync::Arc;

use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Clone)]
pub struct ExternalClient {
    client: Client,
    url: Arc<String>,
}

impl ExternalClient {
    pub fn new(url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            url: Arc::new(url.to_string()),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self) -> Result<T, reqwest::Error> {
        self.client
            .get(self.url.as_ref())
            .send()
            .await?
            .json()
            .await
    }
}
