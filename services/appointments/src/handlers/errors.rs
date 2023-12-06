use axum::{http::StatusCode, response::IntoResponse};
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
    #[error("External client error: {source}")]
    ExternalClientError {
        #[from]
        source: reqwest::Error,
    },
    #[error("Unauthorized")]
    Unauthorized,
    #[error("NotFound")]
    NotFound,
    #[error("Bad Request")]
    BadRequest(String),
}

impl IntoResponse for PublicError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::RepositoryError { source } => {
                <RepositoryError as Into<(StatusCode, String)>>::into(source)
            }
            Self::ValidationError { source } => {
                tracing::error!("{source}");
                (StatusCode::BAD_REQUEST, source.to_string())
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found".into()),
            Self::BadRequest(source) => (StatusCode::BAD_REQUEST, source),
            Self::ExternalClientError { source: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
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
            RepositoryError::ValidationError(source) => {
                (StatusCode::BAD_REQUEST, source.to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".into(),
            ),
        }
    }
}
