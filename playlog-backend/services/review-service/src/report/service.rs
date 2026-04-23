use super::{
    Report, ReportError, ReportRepository, ReportResponse, ReportStatus, ReportTargetType, Result,
};
use crate::{comment::CommentRepository, review::ReviewRepository};
use bson::DateTime;
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

pub struct ReportService {
    report_repository: Box<dyn ReportRepository>,
    review_repository: Box<dyn ReviewRepository>,
    comment_repository: Box<dyn CommentRepository>,
}

impl ReportService {
    pub fn new(
        report_repository: Box<dyn ReportRepository>,
        review_repository: Box<dyn ReviewRepository>,
        comment_repository: Box<dyn CommentRepository>,
    ) -> Self {
        Self {
            report_repository,
            review_repository,
            comment_repository,
        }
    }

    pub async fn report_content(
        &self,
        reporter_id: Uuid,
        reporter_username: String,
        target_type: ReportTargetType,
        target_id: ObjectId,
        reason: String,
    ) -> Result<ReportResponse> {
        if self
            .report_repository
            .find_pending_by_reporter_and_target(reporter_id, target_id, target_type)
            .await?
            .is_some()
        {
            return Err(ReportError::AlreadyReported);
        }

        let report = Report::new(
            target_type,
            target_id,
            reporter_id,
            reporter_username,
            reason,
            DateTime::now(),
        );
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
        resolver_id: Uuid,
        status: ReportStatus,
        version: i64,
    ) -> Result<()> {
        let report = self
            .report_repository
            .find_by_id(id)
            .await?
            .ok_or(ReportError::NotFound)?;

        if report.reporter_id == resolver_id {
            return Err(ReportError::CantResolveOwnReport);
        }

        if status == ReportStatus::Pending {
            return Err(ReportError::IllegalStatus(status.as_db_value()));
        }

        if let ReportStatus::Resolved = status {
            match report.target_type {
                ReportTargetType::Review => {
                    // Try to find the review first to get the version.
                    // If it's already deleted, skip deletion but proceed with report resolution.
                    if let Some(review) =
                        self.review_repository.find_by_id(report.target_id).await?
                    {
                        self.review_repository
                            .delete(report.target_id, review.version)
                            .await?;
                    }
                }
                ReportTargetType::Comment => {
                    // Try to find the comment first to get the version.
                    // If it's already deleted, skip deletion but proceed with report resolution.
                    if let Some(comment) =
                        self.comment_repository.find_by_id(report.target_id).await?
                    {
                        self.comment_repository
                            .delete(report.target_id, comment.version)
                            .await?;
                    }
                }
            }
        }

        self.report_repository
            .update_report_status(id, status, version)
            .await
    }
}
