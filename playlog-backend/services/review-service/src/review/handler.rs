use super::{
    CreateUpdateReviewRequest, GameReviewResponse, GameRatingStatsResponse, ReviewDetailedResponse,
    ReviewQuery, ReviewSimpleResponse,
};
use crate::app::AppState;
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
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let public_routes = OpenApiRouter::new()
        .route("/{id}", get(get_review))
        .route("/game/{game_id}", get(get_reviews_for_game))
        .route("/game/{game_id}/stats", get(get_rating_stats_for_game))
        .route(
            "/user/{user_id}/game/{game_id}",
            get(get_review_for_user_and_game),
        );

    let auth_routes = OpenApiRouter::new()
        .route("/", post(upsert_review))
        .route("/{id}", delete(delete_review))
        .route_layer(from_fn(require_user))
        .route_layer(from_fn_with_state(jwt_config, auth));

    OpenApiRouter::new().merge(public_routes).merge(auth_routes)
}

#[utoipa::path(
    get,
    path = "/api/reviews/{id}",
    summary = "Get review by ID",
    params(("id" = String, Path, description = "Review ObjectId")),
    responses(
        (status = 200, description = "Review found", body = ReviewDetailedResponse),
        (status = 404, description = "Review not found"),
        (status = 400, description = "Invalid ID"),
    ),
    tag = "reviews",
    operation_id = "get_review"
)]
#[debug_handler]
async fn get_review(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ReviewDetailedResponse>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid Review ID"))?;
    let review = state.review_service.get(object_id).await?;
    Ok(Json(review))
}

#[utoipa::path(
    get,
    path = "/api/reviews/game/{game_id}",
    summary = "Get reviews for a game",
    params(
        ("game_id" = i32, Path, description = "Game ID"),
        ReviewQuery
    ),
    responses(
        (status = 200, description = "List of reviews", body = Vec<GameReviewResponse>),
    ),
    tag = "reviews",
    operation_id = "get_reviews_for_game"
)]
#[debug_handler]
async fn get_reviews_for_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i32>,
    Query(query): Query<ReviewQuery>,
) -> Result<Json<Vec<GameReviewResponse>>, ApiError> {
    let reviews = state
        .review_service
        .get_for_game(game_id, query.rating, query.page)
        .await?;
    Ok(Json(reviews))
}

#[utoipa::path(
    get,
    path = "/api/reviews/game/{game_id}/stats",
    summary = "Get total rating count per rating type for a game",
    params(("game_id" = i32, Path, description = "Game ID")),
    responses(
        (status = 200, description = "Stats for the game", body = GameRatingStatsResponse),
    ),
    tag = "reviews",
    operation_id = "get_rating_stats_for_game"
)]
#[debug_handler]
async fn get_rating_stats_for_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i32>,
) -> Result<Json<GameRatingStatsResponse>, ApiError> {
    let stats = state.review_service.get_rating_stats_for_game(game_id).await?;
    Ok(Json(stats))
}

#[utoipa::path(
    get,
    path = "/api/reviews/user/{user_id}/game/{game_id}",
    summary = "Get user's review for a specific game",
    params(
        ("user_id" = Uuid, Path, description = "User UUID"),
        ("game_id" = i32, Path, description = "Game ID"),
    ),
    responses(
        (status = 200, description = "Review found", body = ReviewSimpleResponse),
        (status = 404, description = "Review not found"),
    ),
    tag = "reviews",
    operation_id = "get_review_for_user_and_game"
)]
#[debug_handler]
async fn get_review_for_user_and_game(
    State(state): State<Arc<AppState>>,
    Path((user_id, game_id)): Path<(Uuid, i32)>,
) -> Result<Json<ReviewSimpleResponse>, ApiError> {
    let review = state
        .review_service
        .get_for_user_and_game(user_id, game_id)
        .await?;
    Ok(Json(review))
}

#[utoipa::path(
    post,
    path = "/api/reviews",
    summary = "Create or update a review",
    request_body = CreateUpdateReviewRequest,
    responses(
        (status = 200, description = "Review created or updated", body = ReviewDetailedResponse),
        (status = 400, description = "Invalid request or game not found"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Conflict (already modified)"),
    ),
    tag = "reviews",
    security(("bearer" = [])),
    operation_id = "upsert_review"
)]
#[debug_handler]
async fn upsert_review(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Json(request): Json<CreateUpdateReviewRequest>,
) -> Result<Json<ReviewDetailedResponse>, ApiError> {
    request.validate().map_err(ApiError::from)?;
    let review = state
        .review_service
        .upsert(claims.user_id, claims.username, request)
        .await?;
    Ok(Json(review))
}

#[utoipa::path(
    delete,
    path = "/api/reviews/{id}",
    summary = "Delete own review",
    params(("id" = String, Path, description = "Review ObjectId")),
    responses(
        (status = 204, description = "Review deleted"),
        (status = 400, description = "Invalid ID"),
        (status = 403, description = "Unauthorized (not your review)"),
        (status = 404, description = "Review not found"),
        (status = 409, description = "Conflict (already modified)"),
    ),
    tag = "reviews",
    security(("bearer" = [])),
    operation_id = "delete_review"
)]
#[debug_handler]
async fn delete_review(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Invalid Review ID"))?;
    state
        .review_service
        .delete(claims.user_id, object_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
