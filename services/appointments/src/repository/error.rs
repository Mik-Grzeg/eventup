use std::error::Error;

#[derive(Debug)]
pub enum RepositoryError {
    SQLXDatabase(sqlx::error::ErrorKind),
    SQLXOther,
}

impl Error for RepositoryError {}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Repository Error")
    }
}
