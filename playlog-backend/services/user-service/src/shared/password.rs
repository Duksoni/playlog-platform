use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use validator::ValidationError;

const ERROR_MESSAGE: &str = "Invalid password";

pub fn hash_password(password: &str) -> Result<String, ValidationError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ValidationError::new(ERROR_MESSAGE))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<(), ValidationError> {
    let parsed_hash =
        PasswordHash::new(hashed_password).map_err(|_| ValidationError::new(ERROR_MESSAGE))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ValidationError::new(ERROR_MESSAGE))
}
