import {
	AfterViewInit,
	ChangeDetectionStrategy,
	Component,
	ElementRef,
	inject,
	NgZone,
	OnDestroy,
	OnInit,
	signal,
	ViewChild,
} from '@angular/core';
import {DatePipe} from '@angular/common';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatTooltipModule} from '@angular/material/tooltip';
import {MatChipsModule} from '@angular/material/chips';
import {MatDividerModule} from '@angular/material/divider';
import {ReportService} from '../report.service';
import {ReportResponse, ReportStatus, ReportTargetType} from '../report.dto';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {DialogService} from '../../../shared/services/dialog.service';
import {
	ViewReportedContentDialog
} from '../view-reported-content-dialog/view-reported-content.dialog';

@Component({
	selector: 'app-reports-page',
	imports: [
		DatePipe,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		MatChipsModule,
		MatDividerModule,
	],
	templateUrl: './reports.page.html',
	styleUrl: './reports.page.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ReportsPage implements OnInit, AfterViewInit, OnDestroy {
	@ViewChild('scrollSentinel') set sentinel(el: ElementRef<HTMLElement> | undefined) {
		if (el && this.observer) {
			this.observer.disconnect();
			this.observer.observe(el.nativeElement);
		}
	}

	private reportService = inject(ReportService);
	private snackbarService = inject(SnackbarService);
	private dialogService = inject(DialogService);
	private zone = inject(NgZone);

	protected readonly ReportTargetType = ReportTargetType;

	protected reports = signal<ReportResponse[]>([]);
	protected loading = signal(false);
	protected hasMore = signal(true);
	// Tracks which report id is currently being actioned
	protected actioningId = signal<string | null>(null);

	private page = 0;
	private observer: IntersectionObserver | null = null;
	private readonly pageSize = 10;

	ngOnInit() {
		this.loadFirst();
	}

	ngAfterViewInit() {
		this.setupObserver();
	}

	ngOnDestroy() {
		this.observer?.disconnect();
	}

	private setupObserver() {
		this.zone.runOutsideAngular(() => {
			this.observer = new IntersectionObserver(
				(entries) => {
					if (entries[0].isIntersecting && !this.loading() && this.hasMore()) {
						this.zone.run(() => this.loadNextPage());
					}
				},
				{rootMargin: '100px'},
			);
		});
	}

	private loadFirst() {
		this.page = 0;
		this.hasMore.set(true);
		this.reports.set([]);
		this.loadPage(true);
	}

	private loadPage(replace: boolean) {
		this.loading.set(true);
		this.reportService.getPendingReports(this.page).subscribe({
			next: (data) => {
				this.loading.set(false);
				if (data.length < this.pageSize) this.hasMore.set(false);
				replace ? this.reports.set(data) : this.reports.update(prev => [...prev, ...data]);
			},
			error: () => this.loading.set(false),
		});
	}

	private loadNextPage() {
		if (!this.hasMore() || this.loading()) return;
		this.page++;
		this.loadPage(false);
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
				this.reports.update(prev => prev.filter(r => r.id !== report.id));
				this.snackbarService.createSnackbar(successMsg);
			},
			error: (err) => {
				this.actioningId.set(null);
				if (err.status === 409) {
					this.snackbarService.createSnackbar(
						$localize`:@@reports.conflict:This report was already actioned. Refreshing.`
					);
					this.loadFirst();
				} else {
					this.snackbarService.createSnackbar(
						$localize`:@@reports.actionFailed:Failed to update report.`
					);
				}
			},
		});
	}
}

