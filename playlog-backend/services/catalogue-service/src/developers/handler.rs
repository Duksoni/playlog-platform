use crate::{
    app::AppState,
    entity::{
        CreateGameEntityRequest, GameEntity, GameEntityError, GameEntitySimple, PagedQuery,
        SearchQuery, UpdateGameEntityRequest,
    },
};
use service_common::error::{ApiError, Result as ApiResult};
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
    path = "/api/developers",
    summary = "Get all developers (paged)",
    params(PagedQuery),
    responses(
        (status = 200, description = "List of developers", body = Vec<GameEntitySimple>),
    ),
    tag = "developers",
    operation_id = "get_all_developers_paged"
)]
#[debug_handler]
async fn get_all_paged(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PagedQuery>,
) -> ApiResult<Json<Vec<GameEntitySimple>>> {
    let result = state.developer_repository.get_all_paged(query.page).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/developers/{id}",
    summary = "Get developer by id",
    params(("id" = i32, Path, description = "Developer id")),
    responses(
        (status = 200, description = "Developer", body = GameEntity),
        (status = 404, description = "Developer not found"),
    ),
    tag = "developers",
    operation_id = "get_developer_by_id"
)]
#[debug_handler]
async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<GameEntity>> {
    let result: Option<GameEntity> = state.developer_repository.get(id).await?;
    let result = result.ok_or_else(|| GameEntityError::NotFound(String::from("Developer"), id))?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/developers/search",
    summary = "Search developers by name",
    params(SearchQuery),
    responses(
        (status = 200, description = "Matching developers", body = Vec<GameEntitySimple>),
    ),
    tag = "developers",
    operation_id = "search_developers"
)]
#[debug_handler]
async fn search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<Vec<GameEntitySimple>>> {
    let result = state.developer_repository.find_by_name(&query.q).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/developers",
    summary = "Create a developer (Admin only)",
    request_body = CreateGameEntityRequest,
    responses(
        (status = 201, description = "Developer created", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "developers",
    security(("bearer" = [])),
    operation_id = "create_developer"
)]
#[debug_handler]
async fn create(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateGameEntityRequest>,
) -> ApiResult<impl IntoResponse> {
    request.validate().map_err(ApiError::from)?;
    let result = state.developer_repository.create(&request.name).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    put,
    path = "/api/developers/{id}",
    summary = "Update a developer's name (Admin only)",
    params(("id" = i32, Path, description = "Developer id")),
    request_body = UpdateGameEntityRequest,
    responses(
        (status = 200, description = "Developer updated", body = GameEntity),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Developer not found"),
        (status = 409, description = "Conflict - version mismatch"),
    ),
    tag = "developers",
    security(("bearer" = [])),
    operation_id = "update_developer"
)]
#[debug_handler]
async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateGameEntityRequest>,
) -> ApiResult<Json<GameEntity>> {
    request.validate().map_err(ApiError::from)?;
    let result = state
        .developer_repository
        .update_name(id, &request.name, request.version)
        .await?;
    Ok(Json(result))
}
