pub mod dto;
pub mod error;
pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

pub use dto::{
    CreateUpdateReviewRequest, GameRatingStatsResponse, GameReviewResponse,
    MostReviewedGameResponse, RecentReviewResponse, ReviewDetailedResponse, ReviewQuery,
    ReviewSimpleResponse, TopGameResponse, TopReviewsQuery,
};
pub use error::{Result, ReviewError};
pub use model::{Rating, Review};
pub use repository::{MongoReviewRepository, ReviewRepository};
pub use service::ReviewService;
