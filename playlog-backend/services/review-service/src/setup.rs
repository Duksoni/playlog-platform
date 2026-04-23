use crate::{comment::Comment, report::Report, review::Review};
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

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

    let user_game_unique_active = IndexModel::builder()
        .keys(doc! { "user_id": 1, "game_id": 1 })
        .options(
            IndexOptions::builder()
                .unique(true)
                .partial_filter_expression(doc! { "deleted": false })
                .build(),
        )
        .build();

    reviews_collection
        .create_indexes(vec![game_index, rating_index, user_game_unique_active])
        .await?;

    let comment_target_index = IndexModel::builder()
        .keys(doc! { "target_type": 1, "target_id": 1, "deleted": 1, "created_at": -1 })
        .build();
    comments_collection
        .create_index(comment_target_index)
        .await?;

    let report_status_index = IndexModel::builder().keys(doc! { "status": 1 }).build();
    let report_unique_pending = IndexModel::builder()
        .keys(doc! { "reporter_id": 1, "target_id": 1, "target_type": 1 })
        .options(
            IndexOptions::builder()
                .unique(true)
                .partial_filter_expression(doc! { "status": "PENDING" })
                .build(),
        )
        .build();

    reports_collection
        .create_indexes(vec![report_status_index, report_unique_pending])
        .await?;

    Ok(())
}
