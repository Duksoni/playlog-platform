use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Missing authorization header")]
    MissingAuthorization,

    #[error("Invalid authorization header")]
    InvalidAuthorization,

    #[error("Invalid decoding key: {0}")]
    InvalidDecodingKey(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),
}

pub type Result<T> = std::result::Result<T, JwtError>;
