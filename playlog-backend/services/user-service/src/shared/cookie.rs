use axum::http::{header::SET_COOKIE, HeaderMap};
use cookie::{time::Duration, Cookie};

pub const REFRESH_TOKEN_COOKIE_NAME: &str = "playlog_refresh_token";

pub fn build_cookie_header(
    refresh_token: &str,
    max_age: Duration,
    secure: bool,
    same_site: cookie::SameSite,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let sanitized: String = refresh_token
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
        .collect();
    let cookie = Cookie::build((REFRESH_TOKEN_COOKIE_NAME, sanitized))
        .path("/")
        .max_age(max_age)
        .http_only(true)
        .secure(secure)
        .same_site(same_site)
        .build();
    if let Ok(value) = cookie.to_string().parse() {
        headers.append(SET_COOKIE, value);
    }
    headers
}
