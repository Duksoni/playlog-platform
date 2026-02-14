use api_error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,

    #[error("User is blocked")]
    UserIsBlocked,

    #[error("New password can't be the same as the old one")]
    NewPasswordSameAsOld,

    #[error("Password doesn't match existing one")]
    WrongPassword,

    #[error("Nothing to update. Provide at least one field to update.")]
    NothingToUpdate,

    #[error("Can't block yourself!")]
    CantBlockSelf,

    #[error("Can only deactivate active account. Current status: ${0}")]
    CanOnlyDeactivateActiveAccount(String),

    #[error("Admins can't deactivate their accounts!")]
    AdminCantDeactivateAccount,

    #[error("Can only block active account. Current status: ${0}")]
    CanOnlyBlockActiveAccount(String),

    #[error("Can't promote user from ${0} to ${1}")]
    CantPromote(String, String),

    #[error("You can't promote yourself!")]
    CantPromoteSelf,

    #[error("You can't demote yourself!")]
    CantDemoteSelf,

    #[error("Can't demote from ${0} to ${1}")]
    CantDemote(String, String),

    #[error("Internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, UserError>;

use UserError::*;

impl From<UserError> for ApiError {
    fn from(error: UserError) -> Self {
        let status_code = match error {
            | CantPromote(_, _)
            | CantDemote(_, _)
            | CanOnlyBlockActiveAccount(_)
            | CanOnlyDeactivateActiveAccount(_)
            | CantBlockSelf
            | CantPromoteSelf
            | CantDemoteSelf
            | NewPasswordSameAsOld
            | AdminCantDeactivateAccount
            | NothingToUpdate
            | UserIsBlocked => StatusCode::BAD_REQUEST,
            WrongPassword => StatusCode::UNAUTHORIZED,
            UserNotFound => StatusCode::NOT_FOUND,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError::new(status_code, error.to_string())
    }
}
