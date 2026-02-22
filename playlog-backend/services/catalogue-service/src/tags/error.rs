use thiserror::Error;

#[derive(Debug, Error)]
pub enum TagError {
    #[error("Tag with id {0} not found")]
    NotFound(i32),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, TagError>;
