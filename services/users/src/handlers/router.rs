use crate::app_state::AppState;
use axum::{routing::get, Router};

pub fn router(app_state: AppState) -> Router {
    Router::new().route("/health", get(super::health::health))
    // .route(
    //     "/users",
    //     post(create_handler).post("/api/resource/:id", put(update_handler)),
    // )
    // .route(
    //     "/api/resource/:id",
    //     get(read_handler).delete(delete_handler),
    // )
}
