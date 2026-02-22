use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("Platform with id {0} not found")]
    NotFound(i32),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, PlatformError>;
