use super::{Report, ReportResponse, ReportStatus, ReportTargetType, Result};
use async_trait::async_trait;
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

const PAGE_SIZE: i64 = 10;

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn create_report(&self, report: Report) -> Result<Report>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Report>>;
    async fn find_pending_by_reporter_and_target(
        &self,
        reporter_id: uuid::Uuid,
        target_id: ObjectId,
        target_type: ReportTargetType,
    ) -> Result<Option<Report>>;
    async fn find_pending_reports(&self, page: u64) -> Result<Vec<ReportResponse>>;
    async fn update_report_status(
        &self,
        id: ObjectId,
        status: ReportStatus,
        version: i64,
    ) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct MongoReportRepository {
    reports: Collection<Report>,
}

impl MongoReportRepository {
    pub fn new(reports: Collection<Report>) -> Self {
        Self { reports }
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

    async fn find_pending_by_reporter_and_target(
        &self,
        reporter_id: uuid::Uuid,
        target_id: ObjectId,
        target_type: ReportTargetType,
    ) -> Result<Option<Report>> {
        let uuid_bytes = reporter_id.as_bytes().to_vec();
        let binary = bson::Binary {
            subtype: bson::spec::BinarySubtype::Generic,
            bytes: uuid_bytes,
        };

        let filter = doc! {
            "reporter_id": binary,
            "target_id": target_id,
            "target_type": target_type.as_db_value(),
            "status": ReportStatus::Pending.as_db_value(),
        };

        Ok(self.reports.find_one(filter).await?)
    }

    async fn find_pending_reports(&self, page: u64) -> Result<Vec<ReportResponse>> {
        let skip = (page.max(1) - 1) * PAGE_SIZE as u64;
        let mut cursor = self
            .reports
            .find(doc! { "status": ReportStatus::Pending.as_db_value() })
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

    async fn update_report_status(
        &self,
        id: ObjectId,
        status: ReportStatus,
        version: i64,
    ) -> Result<()> {
        let filter = doc! { "_id": id, "version": version };
        let update = doc! {
            "$set": { "status": status.as_db_value() },
            "$inc": { "version": 1 }
        };
        let result = self.reports.update_one(filter, update).await?;

        if result.matched_count == 0 {
            return Err(super::ReportError::Conflict(id));
        }

        Ok(())
    }
}
