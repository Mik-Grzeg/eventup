#[macro_use]
extern crate pretty_assertions;

use axum::body::Body;
use axum::http;
use axum::http::Method;
use axum::http::Request;
use axum::http::StatusCode;
use common_types::UserIdentifiers;
use common_types::UserRoles;
use http_body_util::BodyExt;
use users_lib::app_state::AppState;

use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use users_lib::handlers::router::router;
use users_lib::types::jwt::LoginTokenRespone;
use users_lib::types::users::UserGet;

use utils::init_tracing;

mod utils;

#[sqlx::test(fixtures("users"))]
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
    let user: UserGet = serde_json::from_slice(&body).unwrap();
    let user_id = user.user_id;

    assert_eq!(user.email, "test_user@mail.com");
    assert_eq!(user.phone_number, "+48123456789");
    assert_eq!(user.first_name.as_deref(), Some("John"));
    assert_eq!(user.last_name.as_deref(), Some("Doe"));
    assert_eq!(user.created_at, user.updated_at);

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
    let login_token: LoginTokenRespone = serde_json::from_slice(&body).unwrap();
    let auth_header = login_token.to_auth_header();

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
    let user_identifiers: UserIdentifiers = serde_json::from_slice(&body).unwrap();

    assert_eq!(user_identifiers.email, "test_user@mail.com");
    assert_eq!(user_identifiers.role, UserRoles::Regular);
    assert_eq!(user_identifiers.id, user_id);

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
    let user: UserGet = serde_json::from_slice(&body).unwrap();

    assert_eq!(user.email, "test_user@mail.com");
    assert_eq!(user.phone_number, "+48123456789");
    assert_eq!(user.first_name.as_deref(), Some("John"));
    assert_eq!(user.last_name.as_deref(), Some("Doe"));
    assert_eq!(user.created_at, user.updated_at);

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
    let user: UserGet = serde_json::from_slice(&body).unwrap();

    assert_eq!(user.email, "test_user@mail.com");
    assert_eq!(user.phone_number, "+48123456789");
    assert_eq!(user.first_name.as_deref(), Some("Johnny")); // changed name
    assert_eq!(user.last_name.as_deref(), Some("Doe"));
    assert_ne!(user.created_at, user.updated_at); // changed updated_at

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

#[sqlx::test(fixtures("users"))]
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
    let login_token: LoginTokenRespone = serde_json::from_slice(&body).unwrap();
    let auth_header = login_token.to_auth_header();

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

#[sqlx::test(fixtures("users"))]
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
    let login_token: LoginTokenRespone = serde_json::from_slice(&body).unwrap();
    let auth_header = login_token.to_auth_header();

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
                        "email": "test_employee@mail.com",
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
    let user: UserGet = serde_json::from_slice(&body).unwrap();

    assert_eq!(user.email, "test_employee@mail.com");
    assert_eq!(user.phone_number, "+48123456789");
    assert_eq!(user.first_name.as_deref(), Some("John"));
    assert_eq!(user.last_name.as_deref(), Some("Doe"));
    assert_eq!(user.created_at, user.updated_at);
    assert_eq!(user.role, UserRoles::Employee);
}

#[sqlx::test(fixtures("users"))]
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
    let login_token: LoginTokenRespone = serde_json::from_slice(&body).unwrap();
    let auth_header = login_token.to_auth_header();

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

#[sqlx::test(fixtures("users"))]
async fn test_selecting_employees_by_regular_user(pool: PgPool) {
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
    let login_token: LoginTokenRespone = serde_json::from_slice(&body).unwrap();
    let auth_header = login_token.to_auth_header();

    // GET employees by regular user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/users/employees")
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}

#[sqlx::test(fixtures("users"))]
async fn test_selecting_employees_by_admin_user(pool: PgPool) {
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
    let login_token: LoginTokenRespone = serde_json::from_slice(&body).unwrap();
    let auth_header = login_token.to_auth_header();

    // GET employees by regular user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/users/employees")
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let users: Vec<UserGet> = serde_json::from_slice(&body).unwrap();

    assert_eq!(users.len(), 2);

    users
        .iter()
        .for_each(|user| assert_eq!(user.role, UserRoles::Employee))
}
