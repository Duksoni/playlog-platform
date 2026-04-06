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
use axum_extra::extract::{Multipart, Query};
use axum_macros::debug_handler;
use jwt_common::{auth, require_admin, JwtConfig};
use utoipa_axum::router::OpenApiRouter;

use crate::dto::{GetGameCoversRequest, GetGameCoversResponse};
use crate::model::FieldName;
use crate::{app::AppState, dto::GameMediaResponse, error::MediaError, model::UploadedFile};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let public_routes = OpenApiRouter::new()
        .route("/games/covers", get(get_game_covers))
        .route("/games/{game_id}", get(get_game_media));

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
    path = "/api/media/games/covers",
    summary = "Get presigned URLs for game covers",
    description = r#"
Accepts an array of game IDs.
Returns a object mapping game IDs to presigned URLs for the game cover image.
If no game covers are found, an empty object is returned.
    "#,
    responses(
        (status = 200, body = GetGameCoversResponse),
    ),
    params(GetGameCoversRequest),
    tag = "multimedia"
)]
#[debug_handler]
async fn get_game_covers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetGameCoversRequest>,
) -> Result<Json<GetGameCoversResponse>, ApiError> {
    if params.game_ids.is_empty() {
        return Ok(Json(GetGameCoversResponse::empty()));
    }
    let cover_map = state
        .media_service
        .get_game_covers_presigned_urls(&params.game_ids)
        .await?;
    Ok(Json(GetGameCoversResponse::new(cover_map)))
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
| `version`    | i64    | -      | Current version of the game data           |

All fields are optional, but at least one must be provided.
Files must include a `Content-Type` header on their part.
    "#,
    params(
        ("game_id" = i32, Path)
    ),
    request_body(
        content_type = "multipart/form-data",
        content = inline(String),
        description = "Multipart form with 'cover', 'screenshot' (repeatable), 'trailer' file fields and 'version' field"
    ),
    responses(
        (status = 200, description = "Upload successful, returns updated media with presigned URLs", body = GameMediaResponse),
        (status = 400, description = "No files provided, unknown field, file too large, missing content-type, or missing version"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - requires admin role"),
        (status = 409, description = "Conflict - version mismatch"),
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
    state.media_service.ensure_game_exists(game_id).await?;
    let mut files: Vec<UploadedFile> = vec![];
    let mut version: Option<i64> = None;

    parse_multipart_request(&mut multipart, &mut files, &mut version).await?;

    let version = version
        .ok_or_else(|| ApiError::new(StatusCode::BAD_REQUEST, "Version field is required"))?;

    let media = state
        .media_service
        .upload_game_media(game_id, files, version)
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

async fn parse_multipart_request(
    multipart: &mut Multipart,
    files: &mut Vec<UploadedFile>,
    version: &mut Option<i64>,
) -> Result<(), ApiError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let field_name = field.name().ok_or_else(|| {
            ApiError::new(StatusCode::BAD_REQUEST, "Multipart field missing name")
        })?;

        if field_name == "version" {
            let game_version: i64 = field
                .text()
                .await
                .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?
                .trim()
                .parse()
                .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid version format"))?;
            *version = Some(game_version);
            continue;
        }

        let field_name_enum = FieldName::from_str(field_name)?;

        let file_name = field.file_name().unwrap_or("upload").to_string();

        let content_type = field
            .content_type()
            .ok_or_else(|| MediaError::MissingContentType(field_name_enum.as_string()))?
            .to_string();

        let data = field
            .bytes()
            .await
            .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

        files.push(UploadedFile::new(
            field_name_enum,
            file_name,
            content_type,
            data,
        ));
    }
    Ok(())
}
