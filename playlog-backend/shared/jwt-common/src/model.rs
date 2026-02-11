use crate::token::ISSUER;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(Error, Debug)]
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
}
