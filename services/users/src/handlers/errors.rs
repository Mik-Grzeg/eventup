use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

use crate::repository::error::RepositoryError;

#[derive(thiserror::Error, Debug)]
pub enum PublicError {
    #[error("Repository Error: {source}")]
    RepositoryError {
        #[from]
        source: RepositoryError,
    },
    #[error("Validation Error: {source}")]
    ValidationError {
        #[from]
        source: ValidationErrors,
    },
}

impl IntoResponse for PublicError {
    fn into_response(self) -> Response {
        match self {
            Self::RepositoryError { source } => {
                tracing::error!("{source}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::ValidationError { source } => {
                tracing::error!("{source}");
                (StatusCode::BAD_REQUEST, "Validation Error: {source}")
            }
        }
        .into_response()
    }
}
