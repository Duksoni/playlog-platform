use api_error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;
use tracing::error;

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
}

pub type Result<T> = std::result::Result<T, LibraryError>;

impl From<LibraryError> for ApiError {
    fn from(error: LibraryError) -> Self {
        let status = match &error {
            LibraryError::NotFound => StatusCode::NOT_FOUND,
            LibraryError::InvalidGameId(_) => StatusCode::BAD_REQUEST,
            LibraryError::DatabaseError(db_err) => {
                error!(error = %db_err, "database error");
                return ApiError::internal_error()
            }
            LibraryError::CatalogueServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError::new(status, error.to_string())
    }
}
