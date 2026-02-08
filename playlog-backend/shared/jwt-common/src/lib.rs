use jsonwebtoken::{errors::Error as JWTError, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;

pub const ISSUER: &str = "https://auth.playlog";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(thiserror::Error, Debug)]
#[error("invalid role: {0}")]
pub struct RoleParseError(String);

impl FromStr for Role {
    type Err = RoleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USER" => Ok(Role::User),
            "MODERATOR" => Ok(Role::Moderator),
            "ADMIN" => Ok(Role::Admin),
            other => Err(RoleParseError(String::from(other))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user UUID)
    pub exp: usize,  // Expiration time (as UTC timestamp)
    pub iat: usize,  // Issued at (as UTC timestamp)
    pub iss: String, // Issuer
    pub role: Option<Role>,
}

impl Claims {
    pub fn for_access_token(sub: String, exp: usize, iat: usize, role: Role) -> Self {
        Claims {
            sub,
            exp,
            iat,
            iss: String::from(ISSUER),
            role: Some(role),
        }
    }

    pub fn for_refresh_token(sub: String, exp: usize, iat: usize) -> Self {
        Claims {
            sub,
            exp,
            iat,
            iss: String::from(ISSUER),
            role: None,
        }
    }
}

pub fn decode_token(token: &str, public_key: &[u8]) -> Result<Claims, JWTError> {
    let decoding_key = DecodingKey::from_rsa_pem(public_key)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.iss = Some(HashSet::from([String::from(ISSUER)]));
    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::path::Path;
    use Role::User;

    fn load_key(path: &str) -> Vec<u8> {
        std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).expect("failed to read key")
    }

    #[test]
    fn decode_token_roundtrip() {
        let private_key = load_key("../../keys/private.pem");
        let public_key = load_key("../../keys/public.pem");

        let now = Utc::now();
        let claims = Claims::for_access_token(
            "user-123".into(),
            (now + Duration::seconds(60)).timestamp() as usize,
            now.timestamp() as usize,
            User,
        );

        let token = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(&private_key).unwrap(),
        )
        .unwrap();

        let decoded = decode_token(&token, &public_key).unwrap();
        assert_eq!(decoded.sub, "user-123");
        assert_eq!(decoded.role, Some(User));
        assert_eq!(decoded.iss, String::from(ISSUER));
    }

    #[test]
    fn decode_with_invalid_issuer_fails() {
        let private_key = load_key("../../keys/private.pem");
        let public_key = load_key("../../keys/public.pem");

        let now = Utc::now();
        let claims = Claims {
            sub: "user-123".into(),
            exp: (now + Duration::seconds(60)).timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: "https://evil.playlog".to_string(),
            role: Some(User),
        };

        let token = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(&private_key).unwrap(),
        )
        .unwrap();

        let err = decode_token(&token, &public_key).unwrap_err();
        assert!(matches!(
            err.kind(),
            jsonwebtoken::errors::ErrorKind::InvalidIssuer
        ));
    }
}
