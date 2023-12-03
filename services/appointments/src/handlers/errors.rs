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
