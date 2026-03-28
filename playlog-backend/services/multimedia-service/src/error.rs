use api_error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MediaError {
    #[error("Media not found for game {0}")]
    NotFound(i32),

    #[error("Game with ID {0} does not exist")]
    InvalidGameId(i32),

    #[error("Catalogue service error: {0}")]
    CatalogueServiceError(String),

    #[error("No files were provided")]
    NoFilesProvided,

    #[error("Unknown multipart field '{0}' - expected 'cover', 'screenshot', or 'trailer'")]
    UnknownField(String),

    #[error("File in field '{field}' exceeds the {limit_mb} MB limit")]
    FileTooLarge { field: String, limit_mb: usize },

    #[error("Missing content-type on field '{0}'")]
    MissingContentType(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Conflict: Version mismatch for game {0}")]
    Conflict(i32),
}

pub type Result<T> = std::result::Result<T, MediaError>;

impl From<MediaError> for ApiError {
    fn from(error: MediaError) -> Self {
        let status = match &error {
            MediaError::NotFound(_) => StatusCode::NOT_FOUND,
            MediaError::InvalidGameId(_) => StatusCode::BAD_REQUEST,
            MediaError::NoFilesProvided
            | MediaError::UnknownField(_)
            | MediaError::FileTooLarge { .. }
            | MediaError::MissingContentType(_) => StatusCode::BAD_REQUEST,
            MediaError::CatalogueServiceError(_)
            | MediaError::DatabaseError(_)
            | MediaError::StorageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MediaError::Conflict(_) => StatusCode::CONFLICT,
        };
        ApiError::new(status, error.to_string())
    }
}
