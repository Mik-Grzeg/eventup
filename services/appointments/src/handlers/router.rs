use std::sync::Arc;

use crate::app_state::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::{
    get_appointments,
    services::{delete_service, get_services, post_service, put_service},
};
use tower_http::trace::TraceLayer;

pub fn router(app_state: AppState) -> Router {
    let health_route = Router::new().route("/health", get(super::health::health));

    let services_router = Router::new()
        .route("/", post(post_service::post_service))
        .route("/", get(get_services::get_services))
        .route("/:id", put(put_service::put_service))
        .route("/:id", delete(delete_service::delete_service))
        .with_state(app_state.clone());

    let appointments_router = Router::new()
        .route("/", get(get_appointments::get_appointments_for_user))
        .with_state(app_state.clone());

    let api_routers = Router::new()
        .nest("/appointments", appointments_router)
        .nest("/services", services_router);

    Router::new()
        .nest("/api/v1", api_routers)
        .merge(health_route)
        .layer(TraceLayer::new_for_http())
}
