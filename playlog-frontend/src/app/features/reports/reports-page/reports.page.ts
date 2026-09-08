import {ChangeDetectionStrategy, Component, inject, OnInit, signal} from '@angular/core';
import {DatePipe} from '@angular/common';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatTooltipModule} from '@angular/material/tooltip';
import {MatTableModule} from '@angular/material/table';
import {MatPaginatorIntl, MatPaginatorModule, PageEvent} from '@angular/material/paginator';
import {ReportService} from '../report.service';
import {ReportResponse, ReportStatus, ReportTargetType} from '../report.dto';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {DialogService} from '../../../shared/services/dialog.service';
import {ViewReportedContentDialog} from '../view-reported-content-dialog/view-reported-content.dialog';
import {PRESET_REASONS} from '../report-dialog/report.dialog';
import {UnknownTotalCountPaginatorIntl} from '../../../shared/unknown-total-count.paginator';

@Component({
	selector: 'app-reports-page',
	imports: [
		DatePipe,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		MatTableModule,
		MatPaginatorModule,
	],
	providers: [
		{provide: MatPaginatorIntl, useClass: UnknownTotalCountPaginatorIntl},
	],
	templateUrl: './reports.page.html',
	styleUrl: './reports.page.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ReportsPage implements OnInit {
	private reportService = inject(ReportService);
	private snackbarService = inject(SnackbarService);
	private dialogService = inject(DialogService);

	protected readonly ReportTargetType = ReportTargetType;
	protected readonly presetReasons = PRESET_REASONS;
	protected readonly displayedColumns = ['type', 'reporter', 'reason', 'createdAt', 'actions'];
	protected readonly detailColumn = ['expandedDetail'];
	protected readonly pageSize = 10;

	protected reports = signal<ReportResponse[]>([]);
	protected loading = signal(false);
	protected pageIndex = signal(0);
	protected totalItems = signal(0);
	protected expandedId = signal<string | null>(null);
	// Tracks which report id is currently being actioned
	protected actioningId = signal<string | null>(null);

	ngOnInit() {
		this.loadPage();
	}

	protected handlePageEvent(event: PageEvent) {
		this.pageIndex.set(event.pageIndex);
		this.expandedId.set(null);
		this.loadPage();
	}

	protected toggleExpand(report: ReportResponse) {
		if (this.presetReasons.includes(report.reason)) return;
		this.expandedId.update(current => current === report.id ? null : report.id);
	}

	protected isCustomReason(report: ReportResponse): boolean {
		return !this.presetReasons.includes(report.reason);
	}

	private loadPage() {
		this.loading.set(true);
		this.reportService.getPendingReports(this.pageIndex()).subscribe({
			next: (data) => {
				if (data.length === 0 && this.pageIndex() > 0) {
					this.pageIndex.update(index => index - 1);
					this.loadPage();
					return;
				}
				this.reports.set(data);
				if (data.length < this.pageSize) {
					this.totalItems.set(this.pageIndex() * this.pageSize + data.length);
				} else {
					this.totalItems.set(Number.MAX_SAFE_INTEGER);
				}
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	protected viewContent(report: ReportResponse) {
		this.dialogService.openDialog(ViewReportedContentDialog, {
			data: {
				targetType: report.targetType,
				targetId: report.targetId,
			},
			width: '600px',
		});
	}

	protected resolve(report: ReportResponse) {
		const dialog = this.dialogService.openSimpleDialog({
			autoFocus: false,
			disableClose: true,
			data: {
				title: $localize`:@@reports.resolveTitle:Resolve Report`,
				content: $localize`:@@reports.resolveConfirm:Are you sure you want to resolve this report? This will hide the reported content.`
			}
		});

		dialog.componentInstance.setPositiveButton(
			$localize`:@@reports.confirmResolve:Resolve`,
			() => this.action(report, ReportStatus.RESOLVED, $localize`:@@reports.resolved:Report resolved.`)
		);
		dialog.componentInstance.setNegativeButton($localize`:@@reports.cancel:Cancel`);
	}

	protected dismiss(report: ReportResponse) {
		const dialog = this.dialogService.openSimpleDialog({
			autoFocus: false,
			disableClose: true,
			data: {
				title: $localize`:@@reports.dismissTitle:Dismiss Report`,
				content: $localize`:@@reports.dismissConfirm:Are you sure you want to dismiss this report? This implies the content does not violate any rules.`
			}
		});

		dialog.componentInstance.setPositiveButton(
			$localize`:@@reports.confirmDismiss:Dismiss`,
			() => this.action(report, ReportStatus.DISMISSED, $localize`:@@reports.dismissed:Report dismissed.`)
		);
		dialog.componentInstance.setNegativeButton($localize`:@@reports.cancel:Cancel`);
	}

	private action(report: ReportResponse, status: ReportStatus, successMsg: string) {
		if (this.actioningId()) return;
		this.actioningId.set(report.id);

		this.reportService.resolveReport(report.id, {status, version: report.version}).subscribe({
			next: () => {
				this.actioningId.set(null);
				if (this.expandedId() === report.id) this.expandedId.set(null);
				this.snackbarService.createSnackbar(successMsg);
				this.loadPage();
			},
			error: (err) => {
				this.actioningId.set(null);
				if (err.status === 409) {
					this.snackbarService.createSnackbar(
						$localize`:@@reports.conflict:This report was already actioned. Refreshing.`
					);
					this.loadPage();
				} else {
					this.snackbarService.createSnackbar(
						$localize`:@@reports.actionFailed:Failed to update report.`
					);
				}
			},
		});
	}
}
