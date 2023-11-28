use bcrypt::BcryptError;
use std::error::Error;

#[derive(Debug)]
pub enum RepositoryError {
    Encryption(String),
    SQLXDatabase(sqlx::error::ErrorKind),
    SQLXOther,
}

impl Error for RepositoryError {}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Repository Error")
    }
}

impl From<BcryptError> for RepositoryError {
    fn from(error: BcryptError) -> Self {
        RepositoryError::Encryption(error.to_string())
    }
}
