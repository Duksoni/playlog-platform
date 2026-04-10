use crate::comment::{DetailedCommentResponse, SimpleCommentResponse, UpdateCommentRequest};
use crate::{
    app::AppState,
    comment::{CommentQuery, CreateCommentRequest},
};
use api_error::ApiError;
use axum::{
    extract::{Path, Query, State}, http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension,
    Json,
};
use axum_macros::debug_handler;
use jwt_common::{auth, require_user, AuthClaims, JwtConfig};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use validator::Validate;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let public_routes = OpenApiRouter::new()
        .route("/", get(get_comments))
        .route("/{id}", get(get_comment));

    let auth_routes = OpenApiRouter::new()
        .route("/", post(add_comment))
        .route("/me/{id}", get(get_own_comment))
        .route("/{id}", put(update_comment))
        .route("/{id}", delete(delete_comment))
        .route_layer(from_fn(require_user))
        .route_layer(from_fn_with_state(jwt_config, auth));

    OpenApiRouter::new().merge(public_routes).merge(auth_routes)
}

#[utoipa::path(
    get,
    path = "/api/comments",
    summary = "Get comments for a game or review",
    params(CommentQuery),
    responses(
        (status = 200, description = "List of comments", body = Vec<SimpleCommentResponse>),
    ),
    tag = "comments",
    operation_id = "get_comments"
)]
#[debug_handler]
async fn get_comments(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CommentQuery>,
) -> Result<Json<Vec<SimpleCommentResponse>>, ApiError> {
    query.validate().map_err(ApiError::from)?;
    let comments = state
        .comment_service
        .get_for_target(query.target_type, &query.target_id, query.page)
        .await?;
    Ok(Json(comments))
}

#[utoipa::path(
    get,
    path = "/api/comments/{id}",
    summary = "Get a comment by ID",
    params(("id" = String, Path, description = "Comment ObjectId")),
    responses(
        (status = 200, description = "Comment found", body = DetailedCommentResponse),
        (status = 400, description = "Invalid ID"),
        (status = 404, description = "Comment not found"),
    ),
    tag = "comments",
    operation_id = "get_comment"
)]
#[debug_handler]
async fn get_comment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DetailedCommentResponse>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid Comment ID"))?;
    let comment = state.comment_service.get(object_id).await?;
    Ok(Json(comment))
}

#[utoipa::path(
    get,
    path = "/api/comments/me/{id}",
    summary = "Get own comment by ID",
    params(("id" = String, Path, description = "Comment ObjectId")),
    responses(
        (status = 200, description = "Comment found", body = DetailedCommentResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Comment not found"),
        (status = 400, description = "Invalid ID"),
    ),
    tag = "comments",
    security(("bearer" = [])),
    operation_id = "get_my_comment"
)]
#[debug_handler]
async fn get_own_comment(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(id): Path<String>,
) -> Result<Json<DetailedCommentResponse>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid Comment ID"))?;
    let comment = state
        .comment_service
        .get_one_for_user(claims.user_id, object_id)
        .await?;
    Ok(Json(comment))
}

#[utoipa::path(
    post,
    path = "/api/comments",
    summary = "Create or update a comment",
    request_body = CreateCommentRequest,
    responses(
        (status = 200, description = "Comment added", body = DetailedCommentResponse),
        (status = 400, description = "Invalid request or target not found"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Conflict (already modified)"),
    ),
    tag = "comments",
    security(("bearer" = [])),
    operation_id = "add_comment"
)]
#[debug_handler]
async fn add_comment(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Json(request): Json<CreateCommentRequest>,
) -> Result<Json<DetailedCommentResponse>, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let comment = state
        .comment_service
        .create(claims.user_id, claims.username, request)
        .await?;
    Ok(Json(comment))
}

#[utoipa::path(
    put,
    path = "/api/comments/{id}",
    summary = "Update own comment text",
    params(("id" = String, Path, description = "Comment ObjectId")),
    request_body = UpdateCommentRequest,
    responses(
        (status = 200, description = "Comment updated", body = DetailedCommentResponse),
        (status = 400, description = "Invalid ID or request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden (not your comment)"),
        (status = 404, description = "Comment not found"),
        (status = 409, description = "Conflict (already modified)"),
    ),
    tag = "comments",
    security(("bearer" = [])),
    operation_id = "update_comment"
)]
#[debug_handler]
async fn update_comment(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(id): Path<String>,
    Json(request): Json<UpdateCommentRequest>,
) -> Result<Json<DetailedCommentResponse>, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid Comment ID"))?;
    let comment = state
        .comment_service
        .update_text(claims.user_id, object_id, request)
        .await?;
    Ok(Json(comment))
}

#[utoipa::path(
    delete,
    path = "/api/comments/{id}",
    summary = "Delete own comment",
    params(("id" = String, Path, description = "Comment ObjectId")),
    responses(
        (status = 204, description = "Comment deleted"),
        (status = 400, description = "Invalid ID"),
        (status = 403, description = "Unauthorized (not your comment)"),
        (status = 404, description = "Comment not found"),
        (status = 409, description = "Conflict (already modified)"),
    ),
    tag = "comments",
    security(("bearer" = [])),
    operation_id = "delete_comment"
)]
#[debug_handler]
async fn delete_comment(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid Comment ID"))?;
    state
        .comment_service
        .delete(claims.user_id, object_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
