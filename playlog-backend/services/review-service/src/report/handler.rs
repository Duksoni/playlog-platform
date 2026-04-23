use crate::report::ReportResponse;
use crate::{
    app::AppState,
    report::{CreateReportRequest, ReportQuery, UpdateReportStatusRequest},
};
use service_common::error::{ApiError, Result as ApiResult};
use axum::{
    extract::{Path, Query, State}, http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{get, post, put},
    Extension,
    Json,
};
use axum_macros::debug_handler;
use jwt_common::{auth, require_moderator, require_user, AuthClaims, JwtConfig};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use validator::Validate;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let jwt_config = JwtConfig::new(state.config.jwt_public_key.clone());

    let user_routes = OpenApiRouter::new()
        .route("/", post(report_content))
        .route_layer(from_fn(require_user));

    let moderator_routes = OpenApiRouter::new()
        .route("/", get(get_report))
        .route("/pending", get(get_pending_reports))
        .route("/{id}/status", put(resolve_report))
        .route_layer(from_fn(require_moderator));

    OpenApiRouter::new()
        .merge(user_routes)
        .merge(moderator_routes)
        .route_layer(from_fn_with_state(jwt_config, auth))
}

#[utoipa::path(
    post,
    path = "/api/reports",
    summary = "Report content",
    request_body = CreateReportRequest,
    responses(
        (status = 200, description = "Report created", body = ReportResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "reports",
    security(("bearer" = [])),
    operation_id = "report_content"
)]
#[debug_handler]
async fn report_content(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AuthClaims>,
    Json(request): Json<CreateReportRequest>,
) -> ApiResult<Json<ReportResponse>> {
    request.validate().map_err(ApiError::from)?;
    let target_id = parse_object_id(&request.target_id, "Invalid target ID")?;
    let report = state
        .report_service
        .report_content(
            claims.user_id,
            claims.username,
            request.target_type,
            target_id,
            request.reason,
        )
        .await?;
    Ok(Json(report))
}

#[utoipa::path(
    get,
    path = "/api/reports/pending",
    summary = "Get pending reports (Moderator and Admin only)",
    params(ReportQuery),
    responses(
        (status = 200, description = "List of pending reports", body = Vec<ReportResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "reports",
    security(("bearer" = [])),
    operation_id = "get_pending_reports"
)]
#[debug_handler]
async fn get_pending_reports(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ReportQuery>,
) -> ApiResult<Json<Vec<ReportResponse>>> {
    let reports = state.report_service.get_pending_reports(query.page).await?;
    Ok(Json(reports))
}

#[utoipa::path(
    get,
    path = "/api/reports/{id}",
    summary = "Get a report",
    params(("id" = String, Path, description = "Report ObjectId")),
    responses(
        (status = 200, description = "Report found", body = ReportResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Report not found"),
    ),
)]
#[debug_handler]
async fn get_report(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<ReportResponse>> {
    let object_id = parse_object_id(&id, "Invalid Report ID")?;
    let report = state.report_service.get_one_pending(object_id).await?;
    Ok(Json(report))
}

#[utoipa::path(
    put,
    path = "/api/reports/{id}/status",
    summary = "Resolve or dismiss a report (Moderator and Admin only)",
    params(("id" = String, Path, description = "Report ObjectId")),
    request_body = UpdateReportStatusRequest,
    responses(
        (status = 204, description = "Report updated"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Report not found"),
        (status = 409, description = "Conflict (already modified)"),
    ),
    tag = "reports",
    security(("bearer" = [])),
    operation_id = "resolve_report"
)]
#[debug_handler]
async fn resolve_report(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Extension(claims): Extension<AuthClaims>,
    Json(request): Json<UpdateReportStatusRequest>,
) -> ApiResult<impl IntoResponse> {
    let object_id = parse_object_id(&id, "Invalid Report ID")?;
    state
        .report_service
        .resolve_report(object_id, claims.user_id, request.status, request.version)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

fn parse_object_id(id: &str, error_message: &str) -> ApiResult<ObjectId> {
    ObjectId::parse_str(id).map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, error_message))
}
