import {ReportTargetType} from '../report.dto';

export interface ReportDialogData {
	targetType: ReportTargetType;
	targetId: string;
	/** Human-readable description shown in the dialog, e.g. "review by username" */
	targetLabel: string;
}
