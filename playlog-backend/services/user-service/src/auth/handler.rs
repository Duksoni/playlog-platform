use super::{AuthError, LoginRequest, RegisterRequest, RegisterResponse, TokenResponse};
use crate::{
    app::AppState,
    shared::{build_cookie_header, REFRESH_TOKEN_COOKIE_NAME},
};
use service_common::error::{ApiError, Result as ApiResult};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};
use axum_extra::extract::cookie::CookieJar;
use axum_macros::debug_handler;
use cookie::time::Duration;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
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
       tag = "auth",
)]
#[debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> ApiResult<impl IntoResponse> {
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
           (status = 201, description = "Successful register", body = RegisterResponse),
           (status = 400, description = "Request has missing values, or the values are invalid"),
           (status = 409, description = "Email or username is already taken"),
       ),
       tag = "auth",
)]
#[debug_handler]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> ApiResult<impl IntoResponse> {
    request.validate().map_err(ApiError::from)?;
    let result = state.auth_service.register(request).await?;
    Ok((StatusCode::CREATED, Json(result)).into_response())
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    summary = "Logout the user",
    responses(
        (status = 204, description = "Logged out"),
        (status = 401, description = "Refresh token missing or invalid"),
    ),
    tag = "auth",
)]
#[debug_handler]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
) -> ApiResult<impl IntoResponse> {
    let refresh_token = extract_refresh_token(&cookie_jar)?;

    let result = state.auth_service.revoke_token(&refresh_token).await;
    if let Err(err) = result {
        tracing::warn!("Failed to revoke refresh token: {}", err);
    }

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
    tag = "auth",
)]
#[debug_handler]
pub async fn refresh_tokens(
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
) -> ApiResult<impl IntoResponse> {
    let refresh_token = extract_refresh_token(&cookie_jar)?;
    let tokens = state
        .auth_service
        .refresh_tokens(&state.config, &refresh_token)
        .await?;

    let cookie_duration = Duration::days(state.config.refresh_token_validity.num_days());
    let headers = build_cookie_header(&tokens.1, cookie_duration);

    let mut response = Json(TokenResponse::new(tokens.0)).into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

fn extract_refresh_token(cookie_jar: &CookieJar) -> ApiResult<String> {
    match cookie_jar.get(REFRESH_TOKEN_COOKIE_NAME) {
        Some(cookie) => Ok(String::from(cookie.value())),
        None => Err(ApiError::from(AuthError::TokenError(String::from(
            "missing refresh token",
        )))),
    }
}
