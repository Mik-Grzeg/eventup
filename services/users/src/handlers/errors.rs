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
    #[error("Unauthenticated")]
    Unauthenticated,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("NotFound")]
    NotFound,
}

impl IntoResponse for PublicError {
    fn into_response(self) -> Response {
        match self {
            Self::RepositoryError { source } => source.into(),
            Self::ValidationError { source } => {
                tracing::error!("{source}");
                (StatusCode::BAD_REQUEST, source.to_string())
            }
            Self::Unauthenticated => (StatusCode::FORBIDDEN, "Unauthenticated".into()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            Self::NotFound => (StatusCode::NOT_FOUND, "NotFound".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".into(),
            ),
        }
        .into_response()
    }
}

impl From<RepositoryError> for (StatusCode, String) {
    fn from(err: RepositoryError) -> Self {
        tracing::error!("{err}");
        match err {
            RepositoryError::SQLXDatabase(sqlx::error::ErrorKind::UniqueViolation) => (
                StatusCode::BAD_REQUEST,
                "Resource with provided identifiers already exists".into(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".into(),
            ),
        }
    }
}

pub enum InternalError {
    Unauthenticated,
}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthenticated => (StatusCode::FORBIDDEN, "Unauthenticated"),
        }
        .into_response()
    }
}
