use chrono::{Duration, NaiveDate, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use validator::ValidationError;

static RE_LOWERCASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z]").unwrap());
static RE_UPPERCASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]").unwrap());
static RE_DIGIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
static RE_SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\W").unwrap());
static RE_NAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\p{L}{1,50}(?:[ '-]\p{L}{1,50})*$").unwrap());

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
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

pub fn validate_first_name(first_name: &str) -> Result<(), ValidationError> {
    if !RE_NAME.is_match(first_name) {
        return Err(ValidationError::new("first_name_invalid"));
    }
    Ok(())
}

pub fn validate_last_name(last_name: &str) -> Result<(), ValidationError> {
    if !RE_NAME.is_match(last_name) {
        return Err(ValidationError::new("last_name_invalid"));
    }
    Ok(())
}

pub fn validate_birthdate_range(date: &NaiveDate) -> Result<(), ValidationError> {
    let today = Utc::now().date_naive();
    // min age: 12, max age: 100
    let max = today - Duration::days(12 * 365);
    let min = today - Duration::days(100 * 365);
    if date < &min || date > &max {
        return Err(ValidationError::new("birthdate_out_of_range"));
    }
    Ok(())
}
