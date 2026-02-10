use super::{AuthError, LoginRequest, RegisterRequest, RegisterResponse, TokenResponse};
use crate::{
    app::AppState,
    shared::{build_cookie_header, REFRESH_TOKEN_COOKIE_NAME},
};
use api_error::ApiError;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};
use axum_extra::extract::cookie::CookieJar;
use axum_macros::debug_handler;
use cookie::time::Duration;
use jwt_common::decode_token;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use uuid::Uuid;
use validator::Validate;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_tokens))
}

#[utoipa::path(
       post,
       path = "/api/auth/login",
       request_body = LoginRequest,
       summary = "Login user",
       responses(
           (status = 200, description = "Authenticated", body = TokenResponse),
           (status = 401, description = "Invalid credentials"),
           (status = 403, description = "User is blocked"),
           (status = 404, description = "Account doesn't exist or it's deactivated"),
       ),
       tag = "Auth",
)]
#[debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let tokens = state.auth_service.login(request, &state.config).await?;

    let cookie_duration = Duration::days(state.config.refresh_token_validity.num_days());
    let headers = build_cookie_header(&tokens.1, cookie_duration);

    let mut response = Json(TokenResponse::new(tokens.0)).into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

#[utoipa::path(
       post,
       path = "/api/auth/register",
       request_body = RegisterRequest,
       summary = "Register new user",
       responses(
           (status = 200, description = "Successful register", body = RegisterResponse),
           (status = 400, description = "Request has missing values, or the values are invalid"),
           (status = 409, description = "Email or username is already taken"),
       ),
       tag = "Auth",
)]
#[debug_handler]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let result = state.auth_service.register(request).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    summary = "Logout the user",
    responses(
        (status = 204, description = "Logged out"),
        (status = 401, description = "Refresh token missing or invalid"),
    ),
    tag = "Auth",
)]
#[debug_handler]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let refresh_token = extract_refresh_token(&cookie_jar)?;

    state.auth_service.revoke_token(&refresh_token).await?;

    let headers = build_cookie_header("", Duration::seconds(0));

    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    summary = "Refresh both tokens",
    responses(
           (status = 200, description = "Refreshed token", body = TokenResponse),
           (status = 401, description = "Refresh token missing or invalid"),
    ),
    tag = "Auth",
)]
#[debug_handler]
pub async fn refresh_tokens(
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let refresh_token = extract_refresh_token(&cookie_jar)?;
    let claims = decode_token(&refresh_token, &state.config.jwt_public_key)
        .map_err(|err| AuthError::TokenError(err.to_string()))?;
    let user_id =
        Uuid::parse_str(&claims.sub).map_err(|err| AuthError::TokenError(err.to_string()))?;
    let tokens = state
        .auth_service
        .refresh_token(&state.config, &refresh_token, user_id)
        .await?;

    let cookie_duration = Duration::days(state.config.refresh_token_validity.num_days());
    let headers = build_cookie_header(&tokens.1, cookie_duration);

    let mut response = Json(TokenResponse::new(tokens.0)).into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

fn extract_refresh_token(cookie_jar: &CookieJar) -> Result<String, ApiError> {
    match cookie_jar.get(REFRESH_TOKEN_COOKIE_NAME) {
        Some(cookie) => Ok(String::from(cookie.value())),
        None => Err(ApiError::from(AuthError::TokenError(String::from(
            "missing refresh token",
        )))),
    }
}
