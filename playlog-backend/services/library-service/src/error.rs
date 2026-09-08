use axum::http::StatusCode;
use service_common::error::{
    is_foreign_key_violation, is_row_not_found, is_unique_violation, ApiError,
};
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
            LibraryError::NotFound | LibraryError::InvalidGameId(_) => StatusCode::NOT_FOUND,
            LibraryError::DatabaseError(db_err) => {
                if is_row_not_found(db_err) {
                    return ApiError::new(StatusCode::NOT_FOUND, "Resource not found");
                }
                if is_unique_violation(db_err) || is_foreign_key_violation(db_err) {
                    return ApiError::new(StatusCode::CONFLICT, db_err.to_string());
                }
                error!(error = %db_err, "database error");
                return ApiError::internal_error();
            }
            LibraryError::CatalogueServiceError(_) => StatusCode::BAD_GATEWAY,
        };
        ApiError::new(status, error.to_string())
    }
}
