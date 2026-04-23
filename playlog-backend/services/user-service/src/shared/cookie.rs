use axum::http::{header::SET_COOKIE, HeaderMap};
use cookie::{time::Duration, Cookie};

pub const REFRESH_TOKEN_COOKIE_NAME: &str = "playlog_refresh_token";

pub fn build_cookie_header(refresh_token: &str, max_age: Duration) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let cookie = Cookie::build((REFRESH_TOKEN_COOKIE_NAME, refresh_token))
        .path("/")
        .max_age(max_age)
        .http_only(true)
        .build();
    headers.append(SET_COOKIE, cookie.to_string().parse().unwrap());
    headers
}
