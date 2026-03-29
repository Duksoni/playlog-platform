use api_error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Game not found in library")]
    NotFound,

    #[error("Game with ID {0} does not exist")]
    InvalidGameId(i32),

    #[error("Catalogue service error: {0}")]
    CatalogueServiceError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, LibraryError>;

impl From<LibraryError> for ApiError {
    fn from(error: LibraryError) -> Self {
        let status = match &error {
            LibraryError::NotFound => StatusCode::NOT_FOUND,
            LibraryError::InvalidGameId(_) => StatusCode::BAD_REQUEST,
            LibraryError::CatalogueServiceError(_)
            | LibraryError::DatabaseError(_)
            | LibraryError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError::new(status, error.to_string())
    }
}
