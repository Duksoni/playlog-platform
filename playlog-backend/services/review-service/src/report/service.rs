use super::{
    Report, ReportError, ReportRepository, ReportResponse, ReportStatus, ReportTargetType, Result,
};
use bson::DateTime;
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

pub struct ReportService {
    report_repository: Box<dyn ReportRepository>,
}

impl ReportService {
    pub fn new(report_repository: Box<dyn ReportRepository>) -> Self {
        Self { report_repository }
    }

    pub async fn report_content(
        &self,
        user_id: Uuid,
        target_type: ReportTargetType,
        target_id: ObjectId,
        reason: String,
    ) -> Result<ReportResponse> {
        let report = Report::new(target_type, target_id, user_id, reason, DateTime::now());
        self.report_repository
            .create_report(report)
            .await
            .map(Report::into)
    }

    pub async fn get_one_pending(&self, id: ObjectId) -> Result<ReportResponse> {
        self.report_repository
            .find_by_id(id)
            .await?
            .map(Report::into)
            .ok_or(ReportError::NotFound)
    }

    pub async fn get_pending_reports(&self, page: u64) -> Result<Vec<ReportResponse>> {
        self.report_repository.find_pending_reports(page).await
    }

    pub async fn resolve_report(
        &self,
        id: ObjectId,
        status: ReportStatus,
        version: i64,
    ) -> Result<()> {
        if status == ReportStatus::Pending {
            return Err(ReportError::IllegalStatus(status.as_string()));
        }
        self.report_repository
            .resolve_report(id, status, version)
            .await
    }
}
