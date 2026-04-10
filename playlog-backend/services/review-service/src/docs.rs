use service_common::docs::SecurityAddon;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "Review Service"),
    paths(
        crate::app::health_check,
        crate::review::handler::get_review,
        crate::review::handler::get_reviews_for_game,
        crate::review::handler::get_rating_stats_for_game,
        crate::review::handler::get_review_for_user_and_game,
        crate::review::handler::upsert_review,
        crate::review::handler::delete_review,
        crate::comment::handler::get_comments,
        crate::comment::handler::get_comment,
        crate::comment::handler::get_own_comment,
        crate::comment::handler::add_comment,
        crate::comment::handler::update_comment,
        crate::comment::handler::delete_comment,
        crate::report::handler::report_content,
        crate::report::handler::get_pending_reports,
        crate::report::handler::resolve_report,
    )
)]
pub struct ApiDoc;
