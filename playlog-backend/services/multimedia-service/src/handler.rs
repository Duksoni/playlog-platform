use std::str::FromStr;
use std::sync::Arc;

use api_error::ApiError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{delete, get, post},
    Json,
};
use axum_extra::extract::Multipart;
use axum_macros::debug_handler;
use jwt_common::{auth, require_admin, JwtConfig};
use utoipa_axum::router::OpenApiRouter;

use crate::{app::AppState, dto::GameMediaResponse, error::MediaError, model::UploadedFile};
use crate::model::FieldName;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let public_routes = OpenApiRouter::new().route("/games/{game_id}", get(get_game_media));

    let admin_routes = OpenApiRouter::new()
        .route("/games/{game_id}/upload", post(upload_game_media))
        .route("/games/{game_id}", delete(delete_game_media))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config, auth));

    OpenApiRouter::new()
        .merge(public_routes)
        .merge(admin_routes)
}

#[utoipa::path(
    get,
    path = "/api/media/games/{game_id}",
    summary = "Get media for a game",
    params(
        ("game_id" = i32, Path)
    ),
    responses(
        (status = 200, description = "Game media with presigned URLs", body = GameMediaResponse),
        (status = 404, description = "No media found for this game"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "multimedia"
)]
#[debug_handler]
async fn get_game_media(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i32>,
) -> Result<Json<GameMediaResponse>, ApiError> {
    let media = state.media_service.get_game_media(game_id).await?;
    Ok(Json(media))
}

#[utoipa::path(
    post,
    path = "/api/media/games/{game_id}/upload",
    summary = "Upload media files for a game (Admin only)",
    description = r#"
Accepts a `multipart/form-data` body with any combination of the following named fields:

| Field name   | Type   | Limit  | Notes                                      |
|--------------|--------|--------|--------------------------------------------|
| `cover`      | image  | 10 MB  | Replaces the existing cover                |
| `screenshot` | image  | 10 MB  | Repeat for multiple; replaces all existing |
| `trailer`    | video  | 500 MB | Replaces the existing trailer              |

All fields are optional, but at least one must be provided.
Files must include a `Content-Type` header on their part.
    "#,
    params(
        ("game_id" = i32, Path)
    ),
    request_body(
        content_type = "multipart/form-data",
        content = inline(String),
        description = "Multipart form with 'cover', 'screenshot' (repeatable), and/or 'trailer' file fields"
    ),
    responses(
        (status = 200, description = "Upload successful, returns updated media with presigned URLs", body = GameMediaResponse),
        (status = 400, description = "No files provided, unknown field, file too large, or missing content-type"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "multimedia",
    security(("bearer" = []))
)]
#[debug_handler]
async fn upload_game_media(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<GameMediaResponse>, ApiError> {
    let mut files: Vec<UploadedFile> = vec![];

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let field_name = field
            .name()
            .ok_or_else(|| ApiError::new(StatusCode::BAD_REQUEST, "Multipart field missing name"))?;
        let field_name = FieldName::from_str(field_name)?;

        let file_name = field.file_name().unwrap_or("upload").to_string();

        let content_type = field
            .content_type()
            .ok_or_else(|| MediaError::MissingContentType(field_name.as_string()))?
            .to_string();

        let data = field
            .bytes()
            .await
            .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

        files.push(UploadedFile::new(field_name, file_name, content_type, data));
    }

    let media = state
        .media_service
        .upload_game_media(game_id, files)
        .await?;

    Ok(Json(media))
}

#[utoipa::path(
    delete,
    path = "/api/media/games/{game_id}",
    summary = "Delete all media for a game (Admin only)",
    params(
        ("game_id" = i32, Path)
    ),
    responses(
        (status = 204, description = "Media deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 404, description = "No media found for this game"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "multimedia",
    security(("bearer" = []))
)]
#[debug_handler]
async fn delete_game_media(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    state.media_service.delete_game_media(game_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
