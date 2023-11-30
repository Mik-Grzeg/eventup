use std::sync::Arc;

use axum::{routing::get, Router};

use crate::app_state::AppState;

use super::get_appointments;
use tower_http::trace::TraceLayer;

pub fn router(app_state: AppState) -> Router {
    let health_route = Router::new().route("/health", get(super::health::health));

    let appointments_router = Router::new()
        .route("/appointments", get(get_appointments::get_appointments_for_user))
        .with_state(app_state);

    Router::new()
        .nest("/api/v1", appointments_router)
        .merge(health_route)
        .layer(TraceLayer::new_for_http())
}
