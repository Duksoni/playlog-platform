import {ChangeDetectionStrategy, Component, inject, signal} from '@angular/core';
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogTitle,
} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {ReportTargetType} from '../report.dto';
import {CommentService} from '../../comments/comment.service';
import {ReviewService} from '../../reviews/review.service';
import {DetailedCommentResponse} from '../../comments/comment.dto';
import {
	RATING_ICONS,
	RATING_LABELS,
	Rating,
	ReviewDetailedResponse
} from '../../reviews/review.dto';
import {Observable} from 'rxjs';
import {DatePipe} from '@angular/common';

export interface ViewReportedContentData {
	targetType: ReportTargetType;
	targetId: string;
}

@Component({
	selector: 'app-view-reported-content-dialog',
	imports: [
		MatDialogTitle,
		MatDialogContent,
		MatDialogActions,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatDialogClose,
		DatePipe,
	],
	templateUrl: './view-reported-content.dialog.html',
	styleUrl: './view-reported-content.dialog.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ViewReportedContentDialog {
	protected data: ViewReportedContentData = inject(MAT_DIALOG_DATA);
	private commentService = inject(CommentService);
	private reviewService = inject(ReviewService);

	protected readonly ReportTargetType = ReportTargetType;
	protected readonly ratingIcons = RATING_ICONS;
	protected readonly ratingLabels = RATING_LABELS;

	protected loading = signal(true);
	protected error = signal<string | null>(null);
	protected contentText = signal<string>('');
	protected authorUsername = signal<string>('');
	protected rating = signal<Rating | null>(null);
	protected createdAt = signal<string | null>(null);
	protected updatedAt = signal<string | null>(null);

	constructor() {
		this.fetchContent();
	}

	private fetchContent() {
		const request: Observable<DetailedCommentResponse | ReviewDetailedResponse> =
			this.data.targetType === ReportTargetType.REVIEW
				? this.reviewService.getReview(this.data.targetId)
				: this.commentService.getComment(this.data.targetId);

		request.subscribe({
			next: (res) => {
				this.loading.set(false);
				this.contentText.set(res.text || '');
				this.authorUsername.set(res.username);
				this.createdAt.set(res.createdAt);
				this.updatedAt.set(res.updatedAt);
				if ('rating' in res) {
					this.rating.set(res.rating);
				}
			},
			error: () => {
				this.loading.set(false);
				this.error.set($localize`:@@reports.fetchError:Failed to fetch reported content. It might have been already deleted.`);
			},
		});
	}

	protected getRatingColorClass(rating: Rating): string {
		switch (rating) {
			case Rating.HIGHLY_RECOMMENDED: return 'rating-highly';
			case Rating.GOOD: return 'rating-good';
			case Rating.OKAY: return 'rating-okay';
			case Rating.NOT_RECOMMENDED: return 'rating-not';
			default: return '';
		}
	}
}
