use axum::async_trait;
use axum::http::StatusCode;
use common_types::UserIdentifiers;

use std::sync::Mutex;

use crate::Authorizable;

#[derive(Default, Debug)]
pub struct MockClient {
    pub get_user_identifiers_to_return: Mutex<Vec<Result<Option<UserIdentifiers>, StatusCode>>>,
    pub called_times: Mutex<i32>,
}

impl MockClient {
    pub fn set_get_identifiers_return(&self, re: Result<Option<UserIdentifiers>, StatusCode>) {
        self.get_user_identifiers_to_return
            .lock()
            .unwrap()
            .push(re.clone());
        self.get_user_identifiers_to_return.lock().unwrap().push(re);
    }
}

#[async_trait]
impl Authorizable for MockClient {
    async fn authorize(&self, _: &str) -> Result<Option<UserIdentifiers>, StatusCode> {
        *self.called_times.lock().unwrap() += 1;
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        extract::{FromRef, State},
        http::{Request, StatusCode},
        middleware::from_extractor_with_state,
        response::IntoResponse,
        routing::get,
        Router,
    };
    use common_types::UserIdentifiers;

    use crate::{mock::MockClient, Authorizable, AuthorizationControl};
    use httpmock::MockServer;
    use tower::ServiceExt;
    use uuid::Uuid;

    #[derive(Clone)]
    struct TestAppState(Arc<dyn Authorizable>);

    impl FromRef<TestAppState> for Arc<dyn Authorizable> {
        fn from_ref(input: &TestAppState) -> Self {
            input.0.clone()
        }
    }

    async fn test_handler(
        AuthorizationControl(user_identifiers): AuthorizationControl,
        State(_): State<TestAppState>,
    ) -> impl IntoResponse {
        println!("Test Handler called");
        if user_identifiers.is_some() {
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    }

    fn app(app_state: TestAppState) -> Router {
        Router::new()
            .route("/test", get(test_handler))
            .route_layer(from_extractor_with_state::<
                AuthorizationControl,
                Arc<dyn Authorizable>,
            >(app_state.0.clone()))
            .with_state(app_state)
    }

    #[tokio::test]
    async fn test_called_only_once() {
        // Arrange
        let mock_client = Arc::new(MockClient::default());

        // Set the expected result for the authorize method
        let user_identifiers_1 = UserIdentifiers {
            id: Uuid::new_v4(),
            email: "test1@mail.com".into(),
            role: common_types::UserRoles::Admin,
        };
        let user_identifiers_2 = UserIdentifiers {
            id: Uuid::new_v4(),
            email: "test2@mail.com".into(),
            role: common_types::UserRoles::Admin,
        };
        mock_client.set_get_identifiers_return(Ok(Some(user_identifiers_1)));
        mock_client.set_get_identifiers_return(Ok(Some(user_identifiers_2)));

        let app_state = TestAppState(mock_client.clone());

        // Create a Router with the AuthorizationControl extractor
        let app = app(app_state);
        // Create a mock server
        let server = MockServer::start();

        // Create a test request with the Authorization header (can be any method or path)
        let test_request = Request::builder()
            .uri(server.url("/test"))
            .header("Authorization", "Bearer valid_token")
            .body(Body::empty())
            .unwrap();

        // Set up the mock response
        server.mock(|when, then| {
            when.path("/test").header_exists("Authorization");
            then.status(200);
        });

        app.oneshot(test_request).await.unwrap();

        // Assert
        // Expect that the authorize method was called once
        // Axum somehow call this extractor twice, hence 2
        assert_eq!(
            mock_client
                .get_user_identifiers_to_return
                .lock()
                .unwrap()
                .len(),
            2
        );
    }
}
