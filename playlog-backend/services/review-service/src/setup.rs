use crate::{comment::Comment, report::Report, review::Review};
use mongodb::{Collection, IndexModel, bson::doc, options::IndexOptions};

pub async fn create_indexes(
    reviews_collection: &Collection<Review>,
    comments_collection: &Collection<Comment>,
    reports_collection: &Collection<Report>,
) -> anyhow::Result<()> {
    let game_index = IndexModel::builder()
        .keys(doc! { "game_id": 1, "deleted": 1, "created_at": -1 })
        .build();

    let rating_index = IndexModel::builder()
        .keys(doc! { "game_id": 1, "rating": 1, "deleted": 1, "created_at": -1 })
        .build();

    let user_game_unique = IndexModel::builder()
        .keys(doc! { "user_id": 1, "game_id": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

    reviews_collection
        .create_indexes(vec![game_index, rating_index, user_game_unique])
        .await?;

    let comment_target_index = IndexModel::builder()
        .keys(doc! { "target_type": 1, "target_id": 1, "deleted": 1, "created_at": -1 })
        .build();
    comments_collection.create_index(comment_target_index).await?;

    let report_status_index = IndexModel::builder().keys(doc! { "status": 1 }).build();
    reports_collection.create_index(report_status_index).await?;

    Ok(())
}
