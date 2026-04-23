export enum ReportTargetType {
	REVIEW = 'REVIEW',
	COMMENT = 'COMMENT',
}

export enum ReportStatus {
	PENDING = 'PENDING',
	RESOLVED = 'RESOLVED',
	DISMISSED = 'DISMISSED',
}

export interface CreateReportRequest {
	targetType: ReportTargetType;
	targetId: string;
	reason: string;
}

export interface ReportResponse {
	id: string;
	targetType: ReportTargetType;
	targetId: string;
	reporterId: string;
	reporterUsername: string;
	reason: string;
	createdAt: string;
	version: number;
}

export interface UpdateReportStatusRequest {
	status: ReportStatus;
	version: number;
}
