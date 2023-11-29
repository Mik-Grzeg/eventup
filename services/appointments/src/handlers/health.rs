use axum::response::IntoResponse;

pub async fn health() -> impl IntoResponse {
    "healthy"
}
