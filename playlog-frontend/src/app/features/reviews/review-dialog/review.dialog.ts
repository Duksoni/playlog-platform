import {ChangeDetectionStrategy, Component, inject, signal} from '@angular/core';
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle,
} from '@angular/material/dialog';
import {FormBuilder, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';

import {ReviewDialogData} from './review-dialog-data';
import {ReviewService} from '../review.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ApiError} from '../../../core/api-error';
import {Rating, RATING_ICONS, RATING_LABELS, RATING_ORDER, ReviewDetailedResponse} from '../review.dto';

@Component({
	selector: 'app-review-dialog',
	imports: [
		ReactiveFormsModule,
		MatDialogTitle,
		MatDialogContent,
		MatDialogActions,
		MatButtonModule,
		MatIconModule,
		MatFormFieldModule,
		MatInputModule,
		MatProgressSpinnerModule,
		MatDialogClose,
	],
	templateUrl: './review.dialog.html',
	styleUrl: './review.dialog.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ReviewDialog {
	protected data: ReviewDialogData = inject(MAT_DIALOG_DATA);
	private dialogRef = inject(MatDialogRef<ReviewDialog>);
	private fb = inject(FormBuilder);
	private reviewService = inject(ReviewService);
	private snackbarService = inject(SnackbarService);

	protected submitting = signal(false);
	protected error = signal<ApiError | null>(null);
	protected selectedRating = signal<Rating | null>(this.data.existing?.rating ?? null);

	protected readonly ratingOrder = RATING_ORDER;
	protected readonly ratingLabels = RATING_LABELS;
	protected readonly ratingIcons = RATING_ICONS;

	protected form = this.fb.group({
		text: [this.data.existing?.text ?? '', Validators.maxLength(10), Validators.maxLength(5000)],
	});

	protected get isEditing(): boolean {
		return !!this.data.existing;
	}

	protected selectRating(rating: Rating) {
		this.selectedRating.set(rating);
	}

	protected onSubmit() {
		const rating = this.selectedRating();
		if (!rating || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		this.reviewService.upsertReview({
			gameId: this.data.gameId,
			rating,
			text: this.form.value.text?.trim() || null,
		}).subscribe({
			next: (review: ReviewDetailedResponse) => {
				this.submitting.set(false);
				this.snackbarService.createSnackbar(
					this.isEditing
						? $localize`:@@review.updated:Review updated.`
						: $localize`:@@review.submitted:Review submitted.`
				);
				this.dialogRef.close(review);
			},
			error: (err) => {
				this.submitting.set(false);
				this.error.set(err as ApiError);
			},
		});
	}

	protected onDelete() {
		const id = this.data.existing?.id;
		if (!id || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		this.reviewService.deleteReview(id).subscribe({
			next: () => {
				this.submitting.set(false);
				this.snackbarService.createSnackbar($localize`:@@review.deleted:Review deleted.`);
				this.dialogRef.close('deleted');
			},
			error: (err) => {
				this.submitting.set(false);
				this.error.set(err as ApiError);
			},
		});
	}
}
