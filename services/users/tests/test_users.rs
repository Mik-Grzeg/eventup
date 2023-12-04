#[macro_use]
extern crate pretty_assertions;

use axum::body::Body;
use axum::http;
use axum::http::Method;
use axum::http::Request;
use axum::http::StatusCode;
use http_body_util::BodyExt;
use users_lib::app_state::AppState;

use serde_json::json;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;
use users_lib::handlers::router::router;
use uuid::Uuid;

use utils::init_tracing;

mod utils;

#[sqlx::test]
async fn test_proper_lifecycle(pool: PgPool) {
    let app_state = AppState::new(pool).await;
    let app = router(app_state);

    init_tracing();

    // POST a new user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/users")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({
                        "email": "test_user@mail.com",
                        "password": "StrongPassword123",
                        "phone_number": "+48123456789",
                        "first_name": "John",
                        "last_name": "Doe",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::CREATED, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let user_id = body
        .get("user_id")
        .and_then(Value::as_str)
        .map(Uuid::parse_str)
        .unwrap()
        .unwrap();
    assert_eq!(
        body.get("email").unwrap().as_str(),
        Some("test_user@mail.com")
    );
    assert_eq!(
        body.get("phone_number").unwrap().as_str(),
        Some("+48123456789")
    );
    assert_eq!(body.get("first_name").unwrap().as_str(), Some("John"));
    assert_eq!(body.get("last_name").unwrap().as_str(), Some("Doe"));
    assert_eq!(body.get("created_at"), body.get("updated_at"));

    // Login as the user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/login")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({
                        "email": "test_user@mail.com",
                        "password": "StrongPassword123",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let (token, r#type) = (body.get("token"), body.get("type"));
    assert!(token.is_some());
    assert!(r#type.is_some());
    let auth_header = format!(
        "{} {}",
        r#type.unwrap().as_str().unwrap(),
        token.unwrap().as_str().unwrap()
    );

    // Access controll
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/access".to_string())
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body.get("email").unwrap().as_str(),
        Some("test_user@mail.com")
    );
    assert_eq!(body.get("role").unwrap().as_str(), Some("regular"));
    assert_eq!(
        body.get("id").unwrap().as_str(),
        Some(user_id.to_string().as_str())
    );

    // GET the user by ID
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/users/{user_id}"))
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body.get("email").unwrap().as_str(),
        Some("test_user@mail.com")
    );
    assert_eq!(
        body.get("phone_number").unwrap().as_str(),
        Some("+48123456789")
    );
    assert_eq!(body.get("first_name").unwrap().as_str(), Some("John"));
    assert_eq!(body.get("last_name").unwrap().as_str(), Some("Doe"));
    assert_eq!(body.get("created_at"), body.get("updated_at"));

    // Update first name of the user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/users/{user_id}"))
                .method(Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::from(
                    json!({
                        "first_name": "Johnny",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body.get("email").unwrap().as_str(),
        Some("test_user@mail.com")
    );
    assert_eq!(
        body.get("phone_number").unwrap().as_str(),
        Some("+48123456789")
    );
    assert_eq!(body.get("first_name").unwrap().as_str(), Some("Johnny"));
    assert_eq!(body.get("last_name").unwrap().as_str(), Some("Doe"));
    assert_ne!(body.get("created_at"), body.get("updated_at"));

    // Delete the user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/users/{user_id}"))
                .method(Method::DELETE)
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::NO_CONTENT, response.status());

    // GET the user by ID (should not be preset | return 404)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/users/{user_id}"))
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[sqlx::test]
async fn test_creating_employee_by_admin(pool: PgPool) {
    let app_state = AppState::new(pool).await;
    let app = router(app_state);

    init_tracing();

    // Login as admin user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/login")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({
                        "email": "admin@mail.com",
                        "password": "dodooooo",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let (token, r#type) = (body.get("token"), body.get("type"));
    assert!(token.is_some());
    assert!(r#type.is_some());
    let auth_header = format!(
        "{} {}",
        r#type.unwrap().as_str().unwrap(),
        token.unwrap().as_str().unwrap()
    );

    // POST a new employee user by admin user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/users")
                .method(Method::POST)
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({
                        "email": "employee1@mail.com",
                        "password": "StrongPassword123",
                        "phone_number": "+48123456789",
                        "first_name": "John",
                        "last_name": "Doe",
                        "role": "employee"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::CREATED, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body.get("email").unwrap().as_str(),
        Some("employee1@mail.com")
    );
    assert_eq!(
        body.get("phone_number").unwrap().as_str(),
        Some("+48123456789")
    );
    assert_eq!(body.get("first_name").unwrap().as_str(), Some("John"));
    assert_eq!(body.get("last_name").unwrap().as_str(), Some("Doe"));
    assert!(body.get("created_at").is_some());
    assert_eq!(body.get("created_at"), body.get("updated_at"));
    assert!(body.get("role").is_none());
}

#[sqlx::test]
async fn test_creating_employee_by_regular_user(pool: PgPool) {
    let app_state = AppState::new(pool).await;
    let app = router(app_state);

    init_tracing();

    // Login as regular user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/login")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({
                        "email": "user@mail.com",
                        "password": "dodooooo",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let (token, r#type) = (body.get("token"), body.get("type"));
    assert!(token.is_some());
    assert!(r#type.is_some());
    let auth_header = format!(
        "{} {}",
        r#type.unwrap().as_str().unwrap(),
        token.unwrap().as_str().unwrap()
    );

    // POST a new employee user by regular user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/users")
                .method(Method::POST)
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({
                        "email": "employee1@mail.com",
                        "password": "StrongPassword123",
                        "phone_number": "+48123456789",
                        "first_name": "John",
                        "last_name": "Doe",
                        "role": "employee"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}

#[sqlx::test]
async fn test_updating_other_user_by_unauthorized_user(pool: PgPool) {
    let app_state = AppState::new(pool).await;
    let app = router(app_state);

    init_tracing();

    // Login as regular user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/login")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    json!({
                        "email": "user@mail.com",
                        "password": "dodooooo",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let (token, r#type) = (body.get("token"), body.get("type"));
    assert!(token.is_some());
    assert!(r#type.is_some());
    let auth_header = format!(
        "{} {}",
        r#type.unwrap().as_str().unwrap(),
        token.unwrap().as_str().unwrap()
    );

    // PUT update an exiting user by other user (should fail)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                // UUID of the other user
                .uri("/api/v1/users/b9ee058b-3143-4176-851b-a60cde9d06ed".to_string())
                .method(Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::from(
                    json!({
                        "first_name": "Johnny",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}
