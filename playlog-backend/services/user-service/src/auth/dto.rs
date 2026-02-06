use chrono::{DateTime, Duration, NaiveDate, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Validate, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub identifier: String, // email or username
    #[validate(length(min = 1, max = 64))]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
}

impl TokenResponse {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[validate(custom(function = "validate_username"))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    #[validate(length(min = 1))]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[validate(length(min = 1))]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[validate(custom(function = "validate_birthdate_range"))]
    pub birthdate: Option<NaiveDate>,
}

impl RegisterRequest {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
            first_name: None,
            last_name: None,
            birthdate: None,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl RegisterResponse {
    pub fn new(id: Uuid, username: String, email: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            username,
            email,
            created_at,
        }
    }
}

static RE_LOWERCASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z]").unwrap());
static RE_UPPERCASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]").unwrap());
static RE_DIGIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
static RE_SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\W").unwrap());
static RE_USERNAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Za-z0-9]+([._][A-Za-z0-9]+)*$").unwrap());

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::new("username_too_short"));
    }

    if username.len() > 16 {
        return Err(ValidationError::new("username_too_long"));
    }

    if !RE_USERNAME.is_match(username) {
        return Err(ValidationError::new("username_invalid"));
    }

    Ok(())
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }

    if password.len() > 64 {
        return Err(ValidationError::new("password_too_long"));
    }

    if !RE_LOWERCASE.is_match(password) {
        return Err(ValidationError::new(
            "password_must_contain_lowercase_letter",
        ));
    }

    if !RE_UPPERCASE.is_match(password) {
        return Err(ValidationError::new(
            "password_must_contain_uppercase_letter",
        ));
    }

    if !RE_DIGIT.is_match(password) {
        return Err(ValidationError::new("password_must_contain_digit"));
    }

    if !RE_SYMBOL.is_match(password) {
        return Err(ValidationError::new("password_must_contain_symbol"));
    }
    Ok(())
}

fn validate_birthdate_range(date: &NaiveDate) -> Result<(), ValidationError> {
    let today = Utc::now().date_naive();
    // min age: 12, max age: 100
    let max = today - Duration::days(12 * 365);
    let min = today - Duration::days(100 * 365);
    if date < &min || date > &max {
        return Err(ValidationError::new("birthdate_out_of_range"));
    }
    Ok(())
}
