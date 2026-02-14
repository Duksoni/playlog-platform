use super::{JwtError, Result, ISSUER};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(Error, Debug)]
#[error("invalid role: {0}")]
pub struct RoleParseError(String);

impl Role {
    pub fn as_db_value(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }

    pub fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl FromStr for Role {
    type Err = RoleParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "USER" => Ok(Role::User),
            "MODERATOR" => Ok(Role::Moderator),
            "ADMIN" => Ok(Role::Admin),
            other => Err(RoleParseError(String::from(other))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| JwtError::InvalidToken(String::from("Invalid UUID for sub claim")))
    }
}

#[derive(Debug, Clone)]
pub struct AccessTokenClaims {
    pub user_id: Uuid,
    pub role: Role,
}

impl TryFrom<Claims> for AccessTokenClaims {
    type Error = JwtError;

    fn try_from(value: Claims) -> Result<Self> {
        let user_id = value.user_id()?;
        let role = value
            .role
            .ok_or(JwtError::InvalidToken(String::from("Missing role in access token")))?;
        Ok(Self { user_id, role })
    }
}

#[derive(Debug, Clone)]
pub struct RefreshTokenClaims {
    pub user_id: Uuid,
}

impl TryFrom<Claims> for RefreshTokenClaims {
    type Error = JwtError;

    fn try_from(value: Claims) -> Result<Self> {
        let user_id = value.user_id()?;
        if value.role.is_some() {
            return Err(JwtError::InvalidToken(String::from(
                "Refresh token cannot have a role",
            )));
        }
        Ok(Self { user_id })
    }
}

#[derive(Debug, Clone)]
pub struct AuthClaims {
    pub user_id: Uuid,
    pub role: Role,
}

impl From<AccessTokenClaims> for AuthClaims {
    fn from(access: AccessTokenClaims) -> Self {
        Self {
            user_id: access.user_id,
            role: access.role,
        }
    }
}

impl TryFrom<Claims> for AuthClaims {
    type Error = JwtError;

    fn try_from(value: Claims) -> Result<Self> {
        AccessTokenClaims::try_from(value).map(AuthClaims::from)
    }
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub public_key: Vec<u8>,
}

impl JwtConfig {
    pub fn new(public_key: Vec<u8>) -> Self {
        Self { public_key }
    }
}
