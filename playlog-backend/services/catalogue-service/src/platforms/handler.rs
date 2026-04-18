use crate::{
    app::AppState,
    entity::{
        CreateGameEntityRequest, GameEntity, GameEntityError, GameEntitySimple, PagedQuery,
        SearchQuery, UpdateGameEntityRequest,
    },
};
use service_common::error::{ApiError, Result as ApiResult};
use axum::routing::delete;
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
        .route("/", get(get_all_paged))
        .route("/search", get(search))
        .route("/{id}", get(get_by_id));

    let admin_routes = OpenApiRouter::new()
        .route("/", post(create))
        .route("/{id}", put(update))
        .route("/{id}", delete(delete_platform))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config, auth));

    OpenApiRouter::new()
        .merge(public_routes)
        .merge(admin_routes)
}

#[utoipa::path(
    get,
    path = "/api/platforms",
    summary = "Get all platforms (paged)",
    params(PagedQuery),
    responses(
        (status = 200, description = "List of platforms", body = Vec<GameEntitySimple>),
    ),
    tag = "platforms",
    operation_id = "get_all_platforms"
)]
#[debug_handler]
async fn get_all_paged(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PagedQuery>,
) -> ApiResult<Json<Vec<GameEntitySimple>>> {
    let result = state.platform_repository.get_all_paged(query.page).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/platforms/{id}",
    summary = "Get platform by id",
    params(("id" = i32, Path, description = "Platform id")),
    responses(
        (status = 200, description = "Platform", body = GameEntity),
        (status = 404, description = "Platform not found"),
    ),
    tag = "platforms",
    operation_id = "get_platform_by_id"
)]
#[debug_handler]
async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<GameEntity>> {
    let result: Option<GameEntity> = state.platform_repository.get(id).await?;
    let result = result.ok_or_else(|| GameEntityError::NotFound(String::from("Platform"), id))?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/platforms/search",
    summary = "Search platforms by name",
    params(SearchQuery),
    responses(
        (status = 200, description = "Matching platforms", body = Vec<GameEntitySimple>),
    ),
    tag = "platforms",
    operation_id = "search_platforms"
)]
#[debug_handler]
async fn search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<Vec<GameEntitySimple>>> {
    let result = state.platform_repository.find_by_name(&query.q).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/platforms",
    summary = "Create a platform (Admin only)",
    request_body = CreateGameEntityRequest,
    responses(
        (status = 201, description = "Platform created", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "platforms",
    security(("bearer" = [])),
    operation_id = "create_platform"
)]
#[debug_handler]
async fn create(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateGameEntityRequest>,
) -> ApiResult<impl IntoResponse> {
    request.validate().map_err(ApiError::from)?;
    let result = state.platform_repository.create(&request.name).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    put,
    path = "/api/platforms/{id}",
    summary = "Update a platform's name (Admin only)",
    params(("id" = i32, Path, description = "Platform id")),
    request_body = UpdateGameEntityRequest,
    responses(
        (status = 200, description = "Platform updated", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Platform not found"),
        (status = 409, description = "Conflict - version mismatch"),
    ),
    tag = "platforms",
    security(("bearer" = [])),
    operation_id = "update_platform"
)]
#[debug_handler]
async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateGameEntityRequest>,
) -> ApiResult<Json<GameEntity>> {
    request.validate().map_err(ApiError::from)?;
    let result = state
        .platform_repository
        .update_name(id, &request.name, request.version)
        .await?;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/api/platforms/{id}",
    summary = "Delete platform",
    params(("id" = String, Path, description = "Platform ID")),
    responses(
        (status = 204, description = "Platform deleted"),
        (status = 404, description = "Platform not found"),
    ),
    tag = "platforms",
    security(("bearer" = [])),
    operation_id = "delete_platform"
)]
#[debug_handler]
async fn delete_platform(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    state.platform_repository.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
