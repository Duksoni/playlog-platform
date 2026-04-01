use super::{Report, ReportError, ReportResponse, ReportStatus, ReportTargetType, Result};
use crate::{comment::Comment, review::Review};
use async_trait::async_trait;
use chrono::Utc;
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId}, Client as MongoDBClient,
    Collection,
};

const PAGE_SIZE: i64 = 10;

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn create_report(&self, report: Report) -> Result<Report>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Report>>;
    async fn find_pending_reports(&self, page: u64) -> Result<Vec<ReportResponse>>;
    async fn resolve_report(&self, id: ObjectId, status: ReportStatus, version: i64) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct MongoReportRepository {
    client: MongoDBClient,
    reports: Collection<Report>,
    reviews: Collection<Review>,
    comments: Collection<Comment>,
}

impl MongoReportRepository {
    pub fn new(
        client: MongoDBClient,
        reports: Collection<Report>,
        reviews: Collection<Review>,
        comments: Collection<Comment>,
    ) -> Self {
        Self {
            client,
            reports,
            reviews,
            comments,
        }
    }
}

#[async_trait]
impl ReportRepository for MongoReportRepository {
    async fn create_report(&self, mut report: Report) -> Result<Report> {
        let result = self.reports.insert_one(report.clone()).await?;
        report.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(report)
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Report>> {
        Ok(self.reports.find_one(doc! { "_id": id }).await?)
    }

    async fn find_pending_reports(&self, page: u64) -> Result<Vec<ReportResponse>> {
        let skip = (page.max(1) - 1) * PAGE_SIZE as u64;
        let mut cursor = self
            .reports
            .find(doc! { "status": ReportStatus::Pending.as_string() })
            .sort(doc! { "created_at": -1 })
            .limit(PAGE_SIZE)
            .skip(skip)
            .await?;
        let mut reports = vec![];
        while let Some(report) = cursor.next().await {
            reports.push(report?.into());
        }
        Ok(reports)
    }

    async fn resolve_report(&self, id: ObjectId, status: ReportStatus, version: i64) -> Result<()> {
        let mut session = self.client.start_session().await?;
        session.start_transaction().await?;

        let report = self
            .reports
            .find_one(doc! { "_id": id })
            .session(&mut session)
            .await?
            .ok_or(ReportError::NotFound)?;

        if let ReportStatus::Resolved = status {
            match report.target_type {
                ReportTargetType::Review => {
                    let filter = doc! { "_id": report.target_id };
                    let update = doc! {
                        "$set": { "deleted": true, "updated_at": mongodb::bson::DateTime::from_millis(Utc::now().timestamp_millis()) },
                        "$inc": { "version": 1 }
                    };
                    self.reviews
                        .update_one(filter, update)
                        .session(&mut session)
                        .await?;
                }
                ReportTargetType::Comment => {
                    let filter = doc! { "_id": report.target_id };
                    let update = doc! { "$set": { "deleted": true } };
                    self.comments
                        .update_one(filter, update)
                        .session(&mut session)
                        .await?;
                }
            }
        }

        let filter = doc! { "_id": id, "version": version };
        let update = doc! {
            "$set": { "status": status.as_string() },
            "$inc": { "version": 1 }
        };
        let result = self
            .reports
            .update_one(filter, update)
            .session(&mut session)
            .await?;

        if result.matched_count == 0 {
            session.abort_transaction().await?;
            return Err(ReportError::Conflict(id));
        }

        session.commit_transaction().await?;
        Ok(())
    }
}
