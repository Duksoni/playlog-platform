use super::{
    CreateGameRequest, Game, GameDetails, GameFilterQuery, GameSimple, GetGamesQuery,
    NewGameReleasesQuery, PublishUnpublishGameRequest, PublsherGamesQuery, UpdateGameRequest,
};
use crate::app::AppState;
use service_common::error::{ApiError, Result as ApiResult};
use axum::{
    extract::{Path, State},
    http::{Extensions, StatusCode},
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json,
};
use axum_extra::extract::Query;
use axum_macros::debug_handler;
use jwt_common::{auth, middleware::auth_optional, require_admin, AuthClaims, JwtConfig, Role};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use validator::Validate;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let public_routes = OpenApiRouter::new()
        .route("/filter", get(filter))
        .route("/by-ids", get(get_games_by_ids))
        .route("/new-releases", get(get_new_releases))
        .route("/{id}", get(get_game))
        .route("/{id}/details", get(get_details))
        .route("/by-developer/{developer_id}", get(find_by_developer))
        .route("/by-publisher/{publisher_id}", get(find_by_publisher))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth_optional));

    let admin_routes = OpenApiRouter::new()
        .route("/", post(create))
        .route("/unpublished", get(get_unpublished))
        .route("/{id}", put(update))
        .route("/{id}", delete(delete_game))
        .route("/{id}/publish", put(publish))
        .route("/{id}/unpublish", put(unpublish))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config, auth));

    OpenApiRouter::new()
        .merge(public_routes)
        .merge(admin_routes)
}

#[utoipa::path(
    get,
    path = "/api/games/filter",
    params(GameFilterQuery),
    summary = "Filter games",
    responses(
        (status = 200, description = "List of games", body = Vec<GameSimple>),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "filter_games"
)]
#[debug_handler]
async fn filter(
    State(state): State<Arc<AppState>>,
    extensions: Extensions,
    Query(params): Query<GameFilterQuery>,
) -> ApiResult<Json<Vec<GameSimple>>> {
    let claims = extensions.get::<AuthClaims>();
    let include_drafts = claims.map(|c| c.role == Role::Admin).unwrap_or(false);

    let games = state.game_service.filter(include_drafts, params).await?;
    Ok(Json(games))
}

#[utoipa::path(
    get,
    path = "/api/games/by-ids",
    params(GetGamesQuery),
    summary = "Get games by ids",
    responses(
        (status = 200, description = "List of games", body = Vec<GameSimple>),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "get_games_by_ids"
)]
#[debug_handler]
async fn get_games_by_ids(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetGamesQuery>,
) -> ApiResult<Json<Vec<GameSimple>>> {
    if query.game_ids.is_empty() {
        return Ok(Json(vec![]));
    }
    let games = state.game_service.get_by_ids(&query.game_ids).await?;
    Ok(Json(games))
}

#[utoipa::path(
    get,
    path = "/api/games/new-releases",
    params(NewGameReleasesQuery),
    summary = "Get new releases",
    responses(
        (status = 200, description = "List of games", body = Vec<GameSimple>),
    ),
    tag = "games",
    operation_id = "get_new_releases"
)]
#[debug_handler]
async fn get_new_releases(
    State(state): State<Arc<AppState>>,
    Query(params): Query<NewGameReleasesQuery>,
) -> ApiResult<Json<Vec<GameSimple>>> {
    let games = state.game_service.get_new_releases(params.limit).await?;
    Ok(Json(games))
}

#[utoipa::path(
    get,
    path = "/api/games/by-developer/{developer_id}",
    summary = "Get games by developer",
    params(("developer_id" = i32, Path, description = "Developer id")),
    responses(
        (status = 200, description = "List of games", body = Vec<GameSimple>),
    ),
    tag = "games",
    operation_id = "get_games_by_developer"
)]
#[debug_handler]
async fn find_by_developer(
    State(state): State<Arc<AppState>>,
    Path(developer_id): Path<i32>,
) -> ApiResult<Json<Vec<GameSimple>>> {
    let games = state.game_service.get_by_developer(developer_id).await?;
    Ok(Json(games))
}

#[utoipa::path(
    get,
    path = "/api/games/by-publisher/{publisher_id}",
    summary = "Get games by publisher",
    params(
        ("publisher_id" = i32, Path, description = "Publisher id"),
        PublsherGamesQuery
    ),
    responses(
        (status = 200, description = "List of games", body = Vec<GameSimple>),
    ),
    tag = "games",
    operation_id = "get_games_by_publisher"
)]
#[debug_handler]
async fn find_by_publisher(
    State(state): State<Arc<AppState>>,
    Path(publisher_id): Path<i32>,
    Query(params): Query<PublsherGamesQuery>,
) -> ApiResult<Json<Vec<GameSimple>>> {
    let games = state
        .game_service
        .get_by_publisher(publisher_id, params.page)
        .await?;
    Ok(Json(games))
}

#[utoipa::path(
    get,
    path = "/api/games/{id}",
    summary = "Get basic game info",
    params(("id" = i32, Path, description = "Game id")),
    responses(
        (status = 200, description = "Game detail with all relations", body = GameSimple),
        (status = 404, description = "Game not found"),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "get_game"
)]
#[debug_handler]
async fn get_game(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<GameSimple>> {
    let game = state.game_service.get(id).await?;
    Ok(Json(game))
}

#[utoipa::path(
    get,
    path = "/api/games/{id}/details",
    summary = "Get full game details",
    params(("id" = i32, Path, description = "Game id")),
    responses(
        (status = 200, description = "Game detail with all relations", body = GameDetails),
        (status = 404, description = "Game not found"),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "get_game_detail"
)]
#[debug_handler]
async fn get_details(
    State(state): State<Arc<AppState>>,
    extensions: Extensions,
    Path(id): Path<i32>,
) -> ApiResult<Json<GameDetails>> {
    let claims = extensions.get::<AuthClaims>();
    let include_draft = claims.map(|c| c.role == Role::Admin).unwrap_or(false);

    let game = state.game_service.get_details(id, include_draft).await?;
    Ok(Json(game))
}

#[utoipa::path(
    get,
    path = "/api/games/unpublished",
    summary = "Get all unpublished games (Admin only)",
    responses(
        (status = 200, description = "Game detail with all relations", body = GameSimple),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "get_unpublished_games"
)]
#[debug_handler]
async fn get_unpublished(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<GameSimple>>> {
    let games = state.game_service.get_all_unpublished().await?;
    Ok(Json(games))
}

#[utoipa::path(
    post,
    path = "/api/games",
    summary = "Create a game (Admin only)",
    request_body = CreateGameRequest,
    responses(
        (status = 201, description = "Game created", body = GameDetails),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "create_game"
)]
#[debug_handler]
async fn create(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateGameRequest>,
) -> ApiResult<impl IntoResponse> {
    request.validate().map_err(ApiError::from)?;
    let game = state.game_service.create(request).await?;
    Ok((StatusCode::CREATED, Json(game)))
}

#[utoipa::path(
    put,
    path = "/api/games/{id}",
    summary = "Update a game (Admin only)",
    params(("id" = i32, Path, description = "Game id")),
    request_body = UpdateGameRequest,
    responses(
        (status = 200, description = "Game updated", body = GameDetails),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Game not found"),
        (status = 409, description = "Conflict - version mismatch"),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "update_game"
)]
#[debug_handler]
async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateGameRequest>,
) -> ApiResult<Json<GameDetails>> {
    request.validate().map_err(ApiError::from)?;
    let game = state.game_service.update(id, request).await?;
    Ok(Json(game))
}

#[utoipa::path(
    delete,
    path = "/api/games/{id}",
    summary = "Delete a game (Admin only)",
    params(("id" = i32, Path, description = "Game id")),
    responses(
        (status = 204, description = "Game deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Game not found"),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "delete_game"
)]
#[debug_handler]
async fn delete_game(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    state.game_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/api/games/{id}/publish",
    summary = "Publish a draft game (Admin only)",
    params(("id" = i32, Path, description = "Game id")),
    request_body = PublishUnpublishGameRequest,
    responses(
        (status = 200, description = "Game published", body = Game),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Game not found"),
        (status = 409, description = "Conflict - version mismatch"),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "publish_game"
)]
#[debug_handler]
async fn publish(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(request): Json<PublishUnpublishGameRequest>,
) -> ApiResult<Json<Game>> {
    request.validate().map_err(ApiError::from)?;
    let game = state.game_service.publish(id, request.version).await?;
    Ok(Json(game))
}

#[utoipa::path(
    put,
    path = "/api/games/{id}/unpublish",
    summary = "Unpublish a game back to draft (Admin only)",
    params(("id" = i32, Path, description = "Game id")),
    request_body = PublishUnpublishGameRequest,
    responses(
        (status = 200, description = "Game unpublished", body = Game),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Game not found"),
        (status = 409, description = "Conflict - version mismatch"),
    ),
    tag = "games",
    security(("bearer" = [])),
    operation_id = "unpublish_game"
)]
#[debug_handler]
async fn unpublish(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(request): Json<PublishUnpublishGameRequest>,
) -> ApiResult<Json<Game>> {
    request.validate().map_err(ApiError::from)?;
    let game = state.game_service.unpublish(id, request.version).await?;
    Ok(Json(game))
}
