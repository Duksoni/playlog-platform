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

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn verify_succeeds_with_correct_password() {
        let password = "P4ssWoRD!?";
        let hashed = hash_password(password).expect("Hashing should succeed");
        assert!(verify_password(password, &hashed).is_ok());
    }

    #[test]
    fn verify_fails_with_incorrect_password() {
        let password = "P4ssWoRD!?";
        let hashed = hash_password(password).expect("Hashing should succeed");
        assert!(verify_password("P4ssWoRD123!?", &hashed).is_err());
    }

    #[test]
    fn hashing_same_password_produces_different_hashes() {
        let password = "!SamePassword123!";
        let first = hash_password(password).expect("Hashing should succeed");
        let second = hash_password(password).expect("Hashing should succeed");
        assert_ne!(first, second, "Each hash should include random salt");
    }

    #[test]
    fn verify_fails_for_malformed_hash() {
        let result = verify_password("P4ssWoRD!?", "invalid-hash");
        assert!(result.is_err());
    }
}
