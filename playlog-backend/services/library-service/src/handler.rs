use crate::{
    app::AppState,
    dto::{AddUpdateGameRequest, LibraryFilterQuery},
    model::{GameLibraryStatus, UserGame}
};
use api_error::ApiError;
use axum::{
    extract::{Path, Query, State}, http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{delete, get, post},
    Extension,
    Json,
};
use axum_macros::debug_handler;
use jwt_common::{auth, require_user, AuthClaims, JwtConfig};
use std::{
    collections::HashMap,
    sync::Arc
};
use utoipa_axum::router::OpenApiRouter;
use uuid::Uuid;
use crate::model::LibraryGame;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let public_routes = OpenApiRouter::new()
        .route("/user/{user_id}", get(get_user_library))
        .route("/user/{user_id}/stats", get(get_library_stats));

    let auth_routes = OpenApiRouter::new()
        .route("/", post(add_or_update_game))
        .route("/{game_id}", delete(remove_from_library))
        .route_layer(from_fn(require_user))
        .route_layer(from_fn_with_state(jwt_config, auth));

    OpenApiRouter::new().merge(public_routes).merge(auth_routes)
}

#[utoipa::path(
    get,
    path = "/api/library/user/{user_id}",
    summary = "Get user's game library",
    params(
        ("user_id" = Uuid, Path, description = "User UUID"),
        LibraryFilterQuery
    ),
    responses(
        (status = 200, description = "List of games in user's library", body = Vec<LibraryGame>),
        (status = 400, description = "Invalid UUID"),
    ),
    tag = "library",
    operation_id = "get_user_library"
)]
#[debug_handler]
pub async fn get_user_library(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Query(filter): Query<LibraryFilterQuery>,
) -> Result<Json<Vec<LibraryGame>>, ApiError> {
    let games = state
        .library_service
        .get_user_library(user_id, filter.status)
        .await?;
    Ok(Json(games))
}

#[utoipa::path(
    get,
    path = "/api/library/user/{user_id}/stats",
    summary = "Get user's library statistics",
    params(("user_id" = Uuid, Path, description = "User UUID")),
    responses(
        (status = 200, description = "Library statistics", body = HashMap<GameLibraryStatus, i64>),
        (status = 400, description = "Invalid UUID"),
    ),
    tag = "library",
    operation_id = "get_library_stats"
)]
#[debug_handler]
pub async fn get_library_stats(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<HashMap<GameLibraryStatus, i64>>, ApiError> {
    let stats = state.library_service.get_library_stats(user_id).await?;
    Ok(Json(stats))
}

#[utoipa::path(
    post,
    path = "/api/library",
    summary = "Add or update a game in the user's library",
    request_body = AddUpdateGameRequest,
    responses(
        (status = 200, description = "Game added or updated", body = UserGame),
        (status = 400, description = "Invalid game ID or request"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "library",
    security(("bearer" = [])),
    operation_id = "add_or_update_game"
)]
#[debug_handler]
pub async fn add_or_update_game(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Json(request): Json<AddUpdateGameRequest>,
) -> Result<Json<UserGame>, ApiError> {
    let game = state
        .library_service
        .add_or_update_game(claims.user_id, request.game_id, request.status)
        .await?;
    Ok(Json(game))
}

#[utoipa::path(
    delete,
    path = "/api/library/{game_id}",
    summary = "Remove a game from the user's library",
    params(("game_id" = i32, Path, description = "Game ID")),
    responses(
        (status = 204, description = "Game removed from library"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Game not found in library"),
    ),
    tag = "library",
    security(("bearer" = [])),
    operation_id = "remove_from_library"
)]
#[debug_handler]
pub async fn remove_from_library(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(game_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .library_service
        .remove_from_library(claims.user_id, game_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
