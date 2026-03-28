use crate::{
    app::AppState,
    entity::{
        CreateGameEntityRequest, GameEntity, GameEntityError, GameEntitySimple, PagedQuery,
        SearchQuery, UpdateGameEntityRequest,
    },
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
        .route("/", get(get_all_paged))
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
    path = "/api/publishers",
    summary = "Get all publishers (paged)",
    params(PagedQuery),
    responses(
        (status = 200, description = "List of publishers", body = Vec<GameEntitySimple>),
    ),
    tag = "publishers",
    operation_id = "get_all_publishers_paged"
)]
#[debug_handler]
pub async fn get_all_paged(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PagedQuery>,
) -> Result<Json<Vec<GameEntitySimple>>, ApiError> {
    let result = state.publisher_repository.get_all_paged(query.page).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/publishers/{id}",
    summary = "Get publisher by id",
    params(("id" = i32, Path, description = "Publisher id")),
    responses(
        (status = 200, description = "Publisher", body = GameEntity),
        (status = 404, description = "Publisher not found"),
    ),
    tag = "publishers",
    operation_id = "get_publisher_by_id"
)]
#[debug_handler]
pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<GameEntity>, ApiError> {
    let result: Option<GameEntity> = state.publisher_repository.get(id).await?;
    let result = result.ok_or_else(|| GameEntityError::NotFound(String::from("Publisher"), id))?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/publishers/search",
    summary = "Search publishers by name",
    params(SearchQuery),
    responses(
        (status = 200, description = "Matching publishers", body = Vec<GameEntitySimple>),
    ),
    tag = "publishers",
    operation_id = "search_publishers"
)]
#[debug_handler]
pub async fn search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<GameEntitySimple>>, ApiError> {
    let result = state.publisher_repository.find_by_name(&query.q).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/publishers",
    summary = "Create a publisher (Admin only)",
    request_body = CreateGameEntityRequest,
    responses(
        (status = 201, description = "Publisher created", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "publishers",
    security(("bearer" = [])),
    operation_id = "create_publisher"
)]
#[debug_handler]
pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateGameEntityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let result = state.publisher_repository.create(&request.name).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    put,
    path = "/api/publishers/{id}",
    summary = "Update a publisher's name (Admin only)",
    params(("id" = i32, Path, description = "Publisher id")),
    request_body = UpdateGameEntityRequest,
    responses(
        (status = 200, description = "Publisher updated", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Publisher not found"),
        (status = 409, description = "Conflict - version mismatch"),
    ),
    tag = "publishers",
    security(("bearer" = [])),
    operation_id = "update_publisher"
)]
#[debug_handler]
pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateGameEntityRequest>,
) -> Result<Json<GameEntity>, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let result = state
        .publisher_repository
        .update_name(id, &request.name, request.version)
        .await?;
    Ok(Json(result))
}
