use crate::{
    comment::CommentService, config::AppConfig, report::ReportService, review::ReviewService,
};

pub struct AppState {
    pub config: AppConfig,
    pub comment_service: CommentService,
    pub report_service: ReportService,
    pub review_service: ReviewService,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        comment_service: CommentService,
        report_service: ReportService,
        review_service: ReviewService,
    ) -> Self {
        Self {
            config,
            comment_service,
            report_service,
            review_service,
        }
    }
}
