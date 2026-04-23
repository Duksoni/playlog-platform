import {
	ChangeDetectionStrategy,
	ChangeDetectorRef,
	Component,
	ElementRef,
	inject,
	input,
	NgZone,
	OnDestroy,
	OnInit,
	signal,
	ViewChild,
} from '@angular/core';
import {FormControl, ReactiveFormsModule, Validators} from '@angular/forms';
import {DatePipe} from '@angular/common';
import {Router} from '@angular/router';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatTooltipModule} from '@angular/material/tooltip';
import {CommentService} from '../../../features/comments/comment.service';
import {CommentTargetType, SimpleCommentResponse} from '../../../features/comments/comment.dto';
import {SessionService} from '../../../core/services/session.service';
import {Role} from '../../../features/auth/auth.dto';
import {SnackbarService} from '../../services/snackbar.service';
import {DialogService} from '../../services/dialog.service';
import {ReportDialog} from '../../../features/reports/report-dialog/report.dialog';
import {ReportTargetType} from '../../../features/reports/report.dto';

@Component({
	selector: 'app-comments-section',
	imports: [
		DatePipe,
		ReactiveFormsModule,
		MatButtonModule,
		MatIconModule,
		MatFormFieldModule,
		MatInputModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
	],
	templateUrl: './comments-section.component.html',
	styleUrl: './comments-section.component.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CommentsSectionComponent implements OnInit, OnDestroy {
	@ViewChild('scrollSentinel') set sentinel(element: ElementRef<HTMLElement> | undefined) {
		if (element && this.observer) {
			this.observer.disconnect();
			this.observer.observe(element.nativeElement);
		}
	}

	targetType = input.required<CommentTargetType>();
	targetId = input.required<string>();

	private commentService = inject(CommentService);
	protected sessionService = inject(SessionService);
	private snackbarService = inject(SnackbarService);
	private dialogService = inject(DialogService);
	private router = inject(Router);
	private zone = inject(NgZone);
	private cd = inject(ChangeDetectorRef);

	protected readonly Role = Role;

	protected comments = signal<SimpleCommentResponse[]>([]);
	protected loading = signal(false);
	protected submitting = signal(false);
	protected editingId = signal<string | null>(null);
	protected expandableComments = signal<Set<string>>(new Set());
	protected expandedComments = signal<Set<string>>(new Set());
	protected reportedCommentIds = signal<Set<string>>(new Set());

	protected composeControl = new FormControl('', [
		Validators.minLength(10),
		Validators.maxLength(2000),
	]);
	protected editControl = new FormControl('', [
		Validators.minLength(10),
		Validators.maxLength(2000),
	]);

	private page = 0;
	private hasMore = true;
	private observer: IntersectionObserver | null = null;
	private readonly pageSize = 10;

	ngOnInit() {
		this.setupObserver();
		this.loadFirst();
	}

	ngOnDestroy() {
		this.observer?.disconnect();
	}

	private setupObserver() {
		this.zone.runOutsideAngular(() => {
			this.observer = new IntersectionObserver(
				(entries) => {
					if (entries[0].isIntersecting && !this.loading() && this.hasMore) {
						this.zone.run(() => this.loadNextPage());
					}
				},
				{rootMargin: '500px'},
			);
		});
	}

	private loadFirst() {
		this.page = 0;
		this.hasMore = true;
		this.comments.set([]);
		this.loadPage(true);
	}

	private loadPage(replace: boolean) {
		this.loading.set(true);
		this.commentService.getComments(this.targetType(), this.targetId(), this.page).subscribe({
			next: (data) => {
				this.loading.set(false);
				if (data.length < this.pageSize) this.hasMore = false;
				replace ? this.comments.set(data) : this.comments.update(prev => [...prev, ...data]);
				setTimeout(() => this.checkAllCommentsOverflow(), 50);
			},
			error: () => this.loading.set(false),
		});
	}

	private loadNextPage() {
		if (!this.hasMore || this.loading()) return;
		this.page++;
		this.loadPage(false);
	}

	protected onSubmit() {
		if (this.composeControl.invalid || this.submitting()) return;
		this.submitting.set(true);

		this.commentService.addComment({
			targetType: this.targetType(),
			targetId: this.targetId(),
			text: this.composeControl.value!.trim(),
		}).subscribe({
			next: (comment) => {
				this.submitting.set(false);
				this.composeControl.reset();
				this.comments.update(prev => [comment, ...prev]);
				setTimeout(() => this.checkAllCommentsOverflow(), 50);
			},
			error: () => {
				this.submitting.set(false);
				this.snackbarService.createSnackbar($localize`:@@comments.submitFailed:Failed to post comment.`);
			},
		});
	}

	protected startEdit(comment: SimpleCommentResponse) {
		this.editingId.set(comment.id);
		this.editControl.setValue(comment.text);
	}

	protected cancelEdit() {
		this.editingId.set(null);
		this.editControl.reset();
	}

	protected onSaveEdit(commentId: string) {
		if (this.editControl.invalid || this.submitting()) return;
		this.submitting.set(true);

		this.commentService.updateComment(commentId, {
			text: this.editControl.value!.trim(),
		}).subscribe({
			next: (updated) => {
				this.submitting.set(false);
				this.editingId.set(null);
				this.editControl.reset();
				this.comments.update(prev => prev.map(c => c.id === commentId ? updated : c));
				setTimeout(() => this.checkAllCommentsOverflow(), 50);
			},
			error: (err) => {
				this.submitting.set(false);
				this.snackbarService.createSnackbar(
					err.status === 409
						? $localize`:@@comments.conflict:This comment was modified. Please refresh.`
						: $localize`:@@comments.updateFailed:Failed to update comment.`
				);
			},
		});
	}

	protected confirmDelete(commentId: string) {
		const dialogRef = this.dialogService.openSimpleDialog({
			width: '400px',
			disableClose: true,
			autoFocus: false,
			data: {
				title: $localize`:@@comments.deleteTitle:Delete Comment`,
				content: $localize`:@@comments.deleteContent:Are you sure you want to delete this comment?`,
			},
		});

		dialogRef.componentInstance.setPositiveButton($localize`:@@common.delete:Delete`, () => {
			this.commentService.deleteComment(commentId).subscribe({
				next: () => {
					this.comments.update(prev => prev.filter(c => c.id !== commentId));
					dialogRef.close();
				},
				error: (err) => {
					dialogRef.close();
					this.snackbarService.createSnackbar(
						err.status === 409
							? $localize`:@@comments.conflict:This comment was modified. Please refresh.`
							: $localize`:@@comments.deleteFailed:Failed to delete comment.`
					);
				},
			});
		});
		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`);
	}

	protected openReportDialog(comment: SimpleCommentResponse) {
		this.dialogService.openDialog(ReportDialog, {
			data: {
				targetType: ReportTargetType.COMMENT,
				targetId: comment.id,
				targetLabel: $localize`:@@report.commentBy:comment by ${comment.username}`,
			},
			width: '500px',
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			if (result) {
				this.reportedCommentIds.update(set => {
					const next = new Set(set);
					next.add(comment.id);
					return next;
				});
			}
		});
	}

	protected isOwnComment(comment: SimpleCommentResponse): boolean {
		return comment.userId === this.sessionService.user().userId;
	}

	protected isCommentExpandable(commentId: string): boolean {
		return this.expandableComments().has(commentId);
	}

	protected isCommentExpanded(commentId: string): boolean {
		return this.expandedComments().has(commentId);
	}

	protected toggleCommentText(commentId: string) {
		this.expandedComments.update(set => {
			const next = new Set(set);
			next.has(commentId) ? next.delete(commentId) : next.add(commentId);
			return next;
		});
	}

	private checkAllCommentsOverflow() {
		const newExpandable = new Set<string>();
		this.comments().forEach(comment => {
			const el = document.getElementById(`comment-text-${comment.id}`);
			if (el && el.scrollHeight > 120) newExpandable.add(comment.id);
		});
		this.expandableComments.set(newExpandable);
		this.cd.markForCheck();
	}

	protected navigateToUser(username: string) {
		this.router.navigate(['/users', username]);
	}

	protected trackById(_: number, item: SimpleCommentResponse): string {
		return item.id;
	}
}
