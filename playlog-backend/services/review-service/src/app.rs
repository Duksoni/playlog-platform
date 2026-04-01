use crate::{config::AppConfig, review::ReviewService};

pub struct AppState {
    pub config: AppConfig,
    pub review_service: ReviewService,
}

impl AppState {
    pub fn new(config: AppConfig, review_service: ReviewService) -> Self {
        Self {
            config,
            review_service,
        }
    }
}
