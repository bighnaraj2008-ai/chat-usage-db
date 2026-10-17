use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    RusqliteError(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Migration error: {0}")]
    MigrationError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Application error: {0}")]
    GenericError(String),
    #[error("Not found: {0}")]
    NotFound(String),
}