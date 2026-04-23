mod app;
mod comment;
mod config;
mod docs;
mod report;
mod review;
mod setup;
mod shared;

use dotenvy::dotenv;
use service_common::{
    http_client::build_client,
    setup::{init_mongodb, init_tracing, shutdown_signal},
};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use app::AppState;
use comment::{CommentService, MongoCommentRepository};
use report::{MongoReportRepository, ReportService};
use review::{MongoReviewRepository, ReviewService};
use setup::create_indexes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing(env!("CARGO_CRATE_NAME"));

    let env = config::load_from_environment()?;

    let http_client = build_client();

    let mongodb_client = init_mongodb(&env.mongodb_uri).await?;
    let database = mongodb_client.database(&env.mongodb_database);

    let reviews_collection = database.collection(&env.reviews_collection);
    let comments_collection = database.collection(&env.comments_collection);
    let reports_collection = database.collection(&env.reports_collection);

    create_indexes(
        &reviews_collection,
        &comments_collection,
        &reports_collection,
    )
    .await?;

    let review_repository = MongoReviewRepository::new(reviews_collection.clone());
    let comment_repository = MongoCommentRepository::new(comments_collection.clone());
    let report_repository = Box::new(MongoReportRepository::new(
        reports_collection,
    ));

    let comment_service = CommentService::new(
        Box::new(comment_repository.clone()),
        Box::new(review_repository.clone()),
        http_client.clone(),
        env.app_config.catalogue_service_url.clone(),
    );

    let report_service = ReportService::new(
        report_repository,
        Box::new(review_repository.clone()),
        Box::new(comment_repository.clone()),
    );

    let review_service = ReviewService::new(
        Box::new(review_repository.clone()),
        http_client.clone(),
        env.app_config.catalogue_service_url.clone(),
    );

    let state = Arc::new(AppState::new(
        env.app_config,
        comment_service,
        report_service,
        review_service,
    ));

    let app = app::build_app(state);

    let server_address = SocketAddr::from(([0, 0, 0, 0], 3004));
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    info!(
        "Review service listening on {}. View docs at http://{}/docs",
        listener.local_addr()?,
        server_address
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
