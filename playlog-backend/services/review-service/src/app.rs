use crate::{comment::CommentService, config::AppConfig, review::ReviewService};

pub struct AppState {
    pub config: AppConfig,
    pub comment_service: CommentService,
    pub review_service: ReviewService,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        comment_service: CommentService,
        review_service: ReviewService,
    ) -> Self {
        Self {
            config,
            comment_service,
            review_service,
        }
    }
}
