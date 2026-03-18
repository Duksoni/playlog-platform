use crate::{
    app::AppState,
    entity::{CreateUpdateGameEntityRequest, GameEntity, GameEntityError, SearchQuery},
};
use api_error::ApiError;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{get, post, put},
    Json,
};
use axum_macros::debug_handler;
use jwt_common::{auth, require_admin, JwtConfig};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use validator::Validate;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let public_routes = OpenApiRouter::new()
        .route("/", get(get_all))
        .route("/search", get(search))
        .route("/{id}", get(get_by_id));

    let admin_routes = OpenApiRouter::new()
        .route("/", post(create))
        .route("/{id}", put(update))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config, auth));

    OpenApiRouter::new()
        .merge(public_routes)
        .merge(admin_routes)
}

#[utoipa::path(
    get,
    path = "/api/genres",
    summary = "Get all genres",
    responses(
        (status = 200, description = "List of genres", body = Vec<GameEntity>),
    ),
    tag = "genres",
    operation_id = "get_all_genres"
)]
#[debug_handler]
pub async fn get_all(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<GameEntity>>, ApiError> {
    let result = state.genre_repository.get_all().await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/genres/{id}",
    summary = "Get genre by id",
    params(("id" = i32, Path, description = "Genre id")),
    responses(
        (status = 200, description = "Genre", body = GameEntity),
        (status = 404, description = "Genre not found"),
    ),
    tag = "genres",
    operation_id = "get_genre_by_id"
)]
#[debug_handler]
pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<GameEntity>, ApiError> {
    let result: Option<GameEntity> = state.genre_repository.get(id).await?;
    let result = result.ok_or_else(|| GameEntityError::NotFound(String::from("Genre"), id))?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/genres/search",
    summary = "Search genres by name",
    params(SearchQuery),
    responses(
        (status = 200, description = "Matching genres", body = Vec<GameEntity>),
    ),
    tag = "genres",
    operation_id = "search_genres"
)]
#[debug_handler]
pub async fn search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<GameEntity>>, ApiError> {
    let result = state.genre_repository.find_by_name(&query.q).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/genres",
    summary = "Create a genre (Admin only)",
    request_body = CreateUpdateGameEntityRequest,
    responses(
        (status = 201, description = "Genre created", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "genres",
    security(("bearer" = [])),
    operation_id = "create_genre"
)]
#[debug_handler]
pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUpdateGameEntityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    req.validate().map_err(ApiError::from)?;
    let result = state.genre_repository.create(&req.name).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    put,
    path = "/api/genres/{id}",
    summary = "Update a genre's name (Admin only)",
    params(("id" = i32, Path, description = "Genre id")),
    request_body = CreateUpdateGameEntityRequest,
    responses(
        (status = 200, description = "Genre updated", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Genre not found"),
    ),
    tag = "genres",
    security(("bearer" = [])),
    operation_id = "update_genre"
)]
#[debug_handler]
pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<CreateUpdateGameEntityRequest>,
) -> Result<Json<GameEntity>, ApiError> {
    req.validate().map_err(ApiError::from)?;
    let result = state.genre_repository.update_name(id, &req.name).await?;
    Ok(Json(result))
}
