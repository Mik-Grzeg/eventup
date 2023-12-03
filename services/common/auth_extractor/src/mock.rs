use axum::async_trait;
use axum::http::StatusCode;
use common_types::UserIdentifiers;
use httpmock::prelude::*;
use std::sync::Mutex;

use crate::Authorizable;

#[derive(Default, Debug)]
pub struct MockClient {
    pub get_user_identifiers_to_return: Mutex<Vec<Result<Option<UserIdentifiers>, StatusCode>>>,
}

impl MockClient {
    pub fn set_get_identifiers_return(&self, re: Result<Option<UserIdentifiers>, StatusCode>) {
        self.get_user_identifiers_to_return.lock().unwrap().push(re);
    }
}

#[async_trait]
impl Authorizable for MockClient {
    async fn authorize(&self, _: &str) -> Result<Option<UserIdentifiers>, StatusCode> {
        let authorization_result = self
            .get_user_identifiers_to_return
            .lock()
            .unwrap()
            .pop()
            .transpose()
            .map(|option| option.flatten());
        authorization_result
    }
}
