use super::{Claims, JwtError, Result};
use axum::http::{header::AUTHORIZATION, HeaderMap};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::collections::HashSet;

pub const ISSUER: &str = "https://auth.playlog";

pub fn extract_bearer_token(headers: &HeaderMap) -> Result<String> {
    let header_value = headers
        .get(AUTHORIZATION)
        .ok_or(JwtError::MissingAuthorization)?
        .to_str()
        .map_err(|err| JwtError::InvalidAuthorizationHeader(err.to_string()))?;

    let token =
        header_value
            .strip_prefix("Bearer ")
            .ok_or(JwtError::InvalidAuthorizationHeader(String::from(
                "Invalid authorization scheme",
            )))?;

    if token.is_empty() {
        return Err(JwtError::InvalidAuthorizationHeader(String::from(
            "Token is empty",
        )));
    }

    Ok(String::from(token))
}

pub fn decode_token(token: &str, public_key: &[u8]) -> Result<Claims> {
    let decoding_key = DecodingKey::from_rsa_pem(public_key)
        .map_err(|err| JwtError::InvalidDecodingKey(err.to_string()))?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.iss = Some(HashSet::from([String::from(ISSUER)]));
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|err| JwtError::InvalidToken(err.to_string()))?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        model::{AccessTokenClaims, RefreshTokenClaims},
        Role::User
    };
    use axum::http::{HeaderMap, HeaderValue};
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::path::Path;

    fn load_key(path: &str) -> Vec<u8> {
        std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).expect("failed to read key")
    }

    #[test]
    fn extract_bearer_token_ok() {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_static("Bearer wrong.jwt.token"),
        );
        assert_eq!(
            extract_bearer_token(&headers).unwrap(),
            "wrong.jwt.token".to_string()
        );
    }

    #[test]
    fn extract_bearer_token_missing_header() {
        let headers = HeaderMap::new();
        let err = extract_bearer_token(&headers).unwrap_err();
        assert!(matches!(err, JwtError::MissingAuthorization));
    }

    #[test]
    fn extract_bearer_token_invalid_scheme() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Token xyz"));
        let err = extract_bearer_token(&headers).unwrap_err();
        assert!(
            matches!(err, JwtError::InvalidAuthorizationHeader(ref message)
        if message == "Invalid authorization scheme")
        );
    }

    #[test]
    fn extract_bearer_token_empty_token() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer "));
        let err = extract_bearer_token(&headers).unwrap_err();
        assert!(
            matches!(err, JwtError::InvalidAuthorizationHeader(ref message)
        if message == "Token is empty")
        );
    }

    #[test]
    fn decode_access_token_roundtrip() {
        let private_key = load_key("../../keys/private.pem");
        let public_key = load_key("../../keys/public.pem");

        let now = Utc::now();
        let claims = Claims::for_access_token(
            "46c7d874-6402-4514-8a0d-d969ba0125d1".to_string(),
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
        assert_eq!(decoded.sub, "46c7d874-6402-4514-8a0d-d969ba0125d1");
        assert_eq!(decoded.role, Some(User));
        assert_eq!(decoded.iss, String::from(ISSUER));

        let access_claims = AccessTokenClaims::try_from(decoded).unwrap();
        assert_eq!(access_claims.role, User);
    }

    #[test]
    fn decode_refresh_token_roundtrip() {
        let private_key = load_key("../../keys/private.pem");
        let public_key = load_key("../../keys/public.pem");

        let now = Utc::now();
        let claims = Claims::for_refresh_token(
            "922e0270-89b1-422a-b396-5dccc3de3b5e".to_string(),
            (now + Duration::seconds(60)).timestamp() as usize,
            now.timestamp() as usize,
        );

        let token = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(&private_key).unwrap(),
        )
        .unwrap();

        let decoded = decode_token(&token, &public_key).unwrap();
        assert_eq!(decoded.role, None);

        // Test conversion to RefreshTokenClaims
        let refresh_claims = RefreshTokenClaims::try_from(decoded).unwrap();
        assert_eq!(
            refresh_claims.user_id.to_string(),
            "922e0270-89b1-422a-b396-5dccc3de3b5e"
        );
    }

    #[test]
    fn decode_with_invalid_issuer_fails() {
        let private_key = load_key("../../keys/private.pem");
        let public_key = load_key("../../keys/public.pem");

        let now = Utc::now();
        let claims = Claims {
            sub: "46c7d874-6402-4514-8a0d-d969ba0125d1".to_string(),
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
            err,
            JwtError::InvalidToken(message) if message.eq("InvalidIssuer"),
        ));
    }
}
