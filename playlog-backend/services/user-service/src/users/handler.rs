use super::{
    FindUsersQuery, FindUsersResponse, UpdatePasswordRequest, UpdateProfileRequest, UserDetails,
    UserError, UserRoleChangeResponse,
};
use crate::app::AppState;
use crate::shared::build_cookie_header;
use api_error::ApiError;
use axum::{
    extract::{Path, Query, State}, http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{delete, get, put},
    Extension,
    Json,
};
use axum_macros::debug_handler;
use cookie::time::Duration;
use jwt_common::{auth, require_admin, require_user, AuthClaims, JwtConfig};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());
    let public_routes = OpenApiRouter::new().route("/{username}", get(get_user));

    let user_routes = OpenApiRouter::new()
        .route("/me", put(update_user))
        .route("/me", delete(deactivate_account))
        .route("/me/change-password", put(change_password))
        .route_layer(from_fn(require_user))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth));

    let admin_routes = OpenApiRouter::new()
        .route("/", get(find_users))
        .route("/{id}/promote", put(promote_user))
        .route("/{id}/demote", put(demote_user))
        .route("/{id}/block", put(block_user))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth));

    OpenApiRouter::new()
        .merge(public_routes)
        .merge(user_routes)
        .merge(admin_routes)

    // Note: The middleware layers are applied in REVERSE order
    // The execution order is:
    // 1. auth middleware runs first (validates JWT, inserts AuthClaims)
    // 2. one of the requre_role middleware runs second
    // 3. handlers run last (with AuthClaims available)
}

#[utoipa::path(
    get,
    path = "/api/users/{username}",
    summary = "Get user profile",
    responses(
        (status = 200, description = "User profile", body = UserDetails),
        (status = 404, description = "User not found"),
    ),
    tag = "users",
)]
#[debug_handler]
async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<UserDetails>, ApiError> {
    let user = state.user_service.get_user_details(user_id).await?;
    Ok(Json(user))
}

#[utoipa::path(
    put,
    path = "/api/users/me",
    summary = "Update user's profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "User updated"),
        (status = 204, description = "Nothing to update (unchanged)"),
        (status = 400, description = "Nothing was provided to update"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    tag = "users",
    security(("bearer" = []))
)]
#[debug_handler]
async fn update_user(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let updated = state
        .user_service
        .update_profile(claims.user_id, request)
        .await?;
    Ok(if updated {
        StatusCode::OK
    } else {
        StatusCode::NO_CONTENT
    })
}

#[utoipa::path(
    put,
    path = "/api/users/me/change-password",
    summary = "Update user's password",
    request_body = UpdatePasswordRequest,
    responses(
        (status = 200, description = "Password updated"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    tag = "users",
    security(("bearer" = []))
)]
#[debug_handler]
async fn change_password(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Json(request): Json<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate().map_err(ApiError::from)?;
    state
        .user_service
        .update_password(claims.user_id, request)
        .await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/api/users/me",
    summary = "Deactivate user's account",
    responses(
        (status = 204, description = "User deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    tag = "users",
    security(("bearer" = []))
)]
#[debug_handler]
async fn deactivate_account(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .user_service
        .deactivate_account(claims.user_id)
        .await?;
    let headers = build_cookie_header("", Duration::seconds(0));

    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}

#[utoipa::path(
    get,
    path = "/api/users",
    summary = "Find users by role and partial username (Admin only)",
    responses(
        (status = 200, description = "List of users", body = FindUsersResponse),
        (status = 400, description = "Invalid query parameters"),
    ),
    params(FindUsersQuery),
    tag = "users",
    security(("bearer" = []))
)]
#[debug_handler]
async fn find_users(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Query(query): Query<FindUsersQuery>,
) -> Result<impl IntoResponse, ApiError> {
    query.validate().map_err(ApiError::from)?;
    let users = state
        .user_service
        .find_users(claims.user_id, &query.partial_username, query.role)
        .await?;
    Ok(Json(users))
}

#[utoipa::path(
    put,
    path = "/api/users/{id}/block",
    summary = "Block user (Admin only)",
    responses(
        (status = 200, description = "User blocked"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 404, description = "User not found"),
    ),
    tag = "users",
    security(("bearer" = []))
)]
#[debug_handler]
async fn block_user(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    if claims.user_id == user_id {
        return Err(UserError::CantBlockSelf.into());
    }
    state.user_service.block_user(user_id).await?;
    Ok(())
}

#[utoipa::path(
    put,
    path = "/api/users/{id}/promote",
    summary = "Promote user to next higher role (Admin only)",
    responses(
        (status = 200, description = "Role updated", body = UserRoleChangeResponse),
        (status = 400, description = "Self promotion, target user is blocked etc."),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 404, description = "User not found")
    ),
    tag = "users",
    security(("bearer" = []))
)]
#[debug_handler]
async fn promote_user(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    if claims.user_id == user_id {
        return Err(UserError::CantPromoteSelf.into());
    }
    let response = state.user_service.promote_user(user_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
    path = "/api/users/{id}/demote",
    summary = "Demote user to next lower role (Admin only)",
    responses(
        (status = 200, description = "Role updated", body = UserRoleChangeResponse),
        (status = 400, description = "Self demotion, target user is blocked etc."),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 404, description = "User not found")
    ),
    tag = "users",
    security(("bearer" = []))
)]
#[debug_handler]
async fn demote_user(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    if claims.user_id == user_id {
        return Err(UserError::CantDemoteSelf.into());
    }
    let response = state.user_service.demote_user(user_id).await?;
    Ok(Json(response))
}
