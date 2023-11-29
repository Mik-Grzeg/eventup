use axum::{http::StatusCode, response::IntoResponse};

pub enum PublicError {}

impl IntoResponse for PublicError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, "Ok").into_response()
    }
}
