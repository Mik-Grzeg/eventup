use axum::{http::StatusCode, response::IntoResponse};

use crate::repository::error::RepositoryError;

#[derive(thiserror::Error, Debug)]
pub enum PublicError {
    #[error("Repository Error: {source}")]
    RepositoryError {
        #[from]
        source: RepositoryError,
    },
}

impl IntoResponse for PublicError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::RepositoryError { source } => {
                <RepositoryError as Into<(StatusCode, String)>>::into(source)
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
