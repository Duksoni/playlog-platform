use super::{AuthError, Result};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, errors::Error as JWTError, Algorithm, EncodingKey, Header};
use jwt_common::{Claims, Role};
use uuid::Uuid;

pub type AccessToken = String;
pub type RefreshToken = String;
pub struct Tokens(pub AccessToken, pub RefreshToken);

pub fn create_tokens(
    access_token_validity: Duration,
    refresh_token_validity: Duration,
    jwt_private_key: &[u8],
    user_id: Uuid,
    role: Role,
) -> Result<(Tokens, DateTime<Utc>)> {
    let now = Utc::now();

    let expiration_date = now + access_token_validity;
    let claims = Claims::for_access_token(
        user_id.to_string(),
        expiration_date.timestamp() as usize,
        now.timestamp() as usize,
        role,
    );
    let access_token = create_token(claims, &jwt_private_key)?;

    let expiration_date = now + refresh_token_validity;
    let claims = Claims::for_refresh_token(
        user_id.to_string(),
        expiration_date.timestamp() as usize,
        now.timestamp() as usize,
    );
    let refresh_token = create_token(claims, &jwt_private_key)?;

    Ok((Tokens(access_token, refresh_token), expiration_date))
}

fn create_token(claims: Claims, jwt_private_key: &[u8]) -> Result<String> {
    let header = Header::new(Algorithm::RS256);
    let encoding_key = EncodingKey::from_rsa_pem(jwt_private_key)?;
    encode(&header, &claims, &encoding_key).map_err(Into::into)
}

impl From<JWTError> for AuthError {
    fn from(err: JWTError) -> Self {
        AuthError::TokenError(err.to_string())
    }
}
