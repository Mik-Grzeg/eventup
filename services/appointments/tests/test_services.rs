#[macro_use]
extern crate pretty_assertions;

use std::sync::Arc;

use appointments_lib::app_state::AppState;
use appointments_lib::handlers::router::router;
use appointments_lib::types::services::ServiceGet;
use axum::body::Body;
use axum::http;
use axum::http::Method;
use axum::http::Request;
use axum::http::StatusCode;

use http_body_util::BodyExt;

use serde_json::json;

use sqlx::PgPool;
use tower::ServiceExt;

use utils::{admin_user_identifiers, init_tracing, regular_user_identifiers};

use auth_extractor::mock::MockClient;

mod utils;

#[sqlx::test]
async fn test_creating_service_by_admin(pool: PgPool) {
    let access = Arc::new(MockClient::default());
    let app_state = AppState::new(pool)
        .await
        .with_access_control(access.clone());
    let app = router(app_state);

    init_tracing();

    access.set_get_identifiers_return(Ok(Some(admin_user_identifiers())));
    // POST a new active service
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, "")
                .body(Body::from(
                    json!({
                        "name": "Test active service",
                        "description": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s",
                        "duration_in_sec": 60 * 30,
                        "price": 10.0,
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::CREATED, response.status());

    access.set_get_identifiers_return(Ok(Some(admin_user_identifiers())));
    // POST a new inactive service
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, "")
                .body(Body::from(
                    json!({
                        "name": "Test inactive service",
                        "description": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s",
                        "duration_in_sec": 60 * 60,
                        "price": 100.0,
                        "active": false
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::CREATED, response.status());

    // View as admin
    access.set_get_identifiers_return(Ok(Some(admin_user_identifiers())));
    // GET check if both services were created properly the services
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services")
                .method(Method::GET)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, "")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let services: Vec<ServiceGet> = serde_json::from_slice(&body).unwrap();

    assert_eq!(services.len(), 2);
    // Check prices of both services
    assert_eq!(services[0].duration_in_sec, 60 * 30);
    assert_eq!(services[1].duration_in_sec, 60 * 60);

    // Check names of both services
    assert_eq!(services[0].name, "Test active service");
    assert_eq!(services[1].name, "Test inactive service");

    // Check prices of both services
    assert_eq!(services[0].price, 10.0);
    assert_eq!(services[1].price, 100.0);

    // Check activity of both services
    assert!(services[0].active);
    assert!(!services[1].active);

    // View as regular user
    access.set_get_identifiers_return(Ok(Some(regular_user_identifiers())));
    // GET check if both services were created properly the services
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services")
                .method(Method::GET)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, "")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let services: Vec<ServiceGet> = serde_json::from_slice(&body).unwrap();

    assert_eq!(services.len(), 1);
    assert_eq!(services[0].duration_in_sec, 60 * 30);
    assert_eq!(services[0].name, "Test active service");
    assert_eq!(services[0].price, 10.0);
    assert_eq!(services[0].updated_at, services[0].created_at);
    assert!(services[0].active);
}

#[sqlx::test]
async fn test_creating_service_by_regular_user(pool: PgPool) {
    let access = Arc::new(MockClient::default());
    let app_state = AppState::new(pool)
        .await
        .with_access_control(access.clone());
    let app = router(app_state.clone());

    init_tracing();

    access.set_get_identifiers_return(Ok(Some(regular_user_identifiers())));
    // POST a new active service
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services")
                .method(Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, "")
                .body(Body::from(
                    json!({
                        "name": "Test active service",
                        "description": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s",
                        "duration_in_sec": 60 * 30,
                        "price": 10.0,
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}

#[sqlx::test(fixtures("services.sql"))]
async fn test_updating_service_by_regular_user(pool: PgPool) {
    let access = Arc::new(MockClient::default());
    let app_state = AppState::new(pool)
        .await
        .with_access_control(access.clone());
    let app = router(app_state.clone());

    init_tracing();

    access.set_get_identifiers_return(Ok(Some(regular_user_identifiers())));
    // PUT update service
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services/00000000-0000-0000-0000-000000000001")
                .method(Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, "")
                .body(Body::from(
                    json!({
                        "name": "Test active service",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
}

#[sqlx::test(fixtures("services.sql"))]
async fn test_updating_service_by_admin_user(pool: PgPool) {
    let access = Arc::new(MockClient::default());
    let app_state = AppState::new(pool)
        .await
        .with_access_control(access.clone());
    let app = router(app_state.clone());

    init_tracing();

    access.set_get_identifiers_return(Ok(Some(admin_user_identifiers())));
    // PUT update service
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services/00000000-0000-0000-0000-000000000001")
                .method(Method::PUT)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header(http::header::AUTHORIZATION, "")
                .body(Body::from(
                    json!({
                        "name": "New Test active service",
                        "price": 37.0
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());

    // GET updated service
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/services/00000000-0000-0000-0000-000000000001")
                .method(Method::GET)
                .header(http::header::AUTHORIZATION, "")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let service: ServiceGet = serde_json::from_slice(&body).unwrap();

    assert_eq!(service.duration_in_sec, 60 * 60);
    assert_eq!(service.name, "New Test active service"); // updated service name
    assert_eq!(service.price, 37.0); // updated price
    assert_ne!(service.created_at, service.updated_at); // updated updated_at
    assert!(service.active);
}
