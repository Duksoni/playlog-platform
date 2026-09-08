import {ChangeDetectionStrategy, Component, inject, signal} from '@angular/core';
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle,
} from '@angular/material/dialog';
import {FormControl, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {ReportDialogData} from './report-dialog-data';
import {ReportService} from '../report.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ApiError} from '../../../core/api-error';
import {ReportTargetType} from '../report.dto';

export const PRESET_REASONS: string[] = [
	$localize`:@@report.reasonHate:Hate speech or harassment`,
	$localize`:@@report.reasonPrivacy:Privacy violation`,
	$localize`:@@report.reasonSpam:Spam or advertising`,
	$localize`:@@report.reasonInappropriate:Inappropriate content`,
	$localize`:@@report.reasonOffTopic:Off-topic content`,
];

const OTHER = $localize`:@@report.reasonOther:Other`;

@Component({
	selector: 'app-report-dialog',
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
	templateUrl: './report.dialog.html',
	styleUrl: './report.dialog.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ReportDialog {
	protected data: ReportDialogData = inject(MAT_DIALOG_DATA);
	private dialogRef = inject(MatDialogRef<ReportDialog>);
	private reportService = inject(ReportService);
	private snackbarService = inject(SnackbarService);

	protected readonly presetReasons = PRESET_REASONS;
	protected readonly other = OTHER;
	protected readonly ReportTargetType = ReportTargetType;

	protected submitting = signal(false);
	protected error = signal<ApiError | null>(null);
	protected selectedPreset = signal<string | null>(null);
	protected showCustomReason = signal(false);

	protected reasonControl = new FormControl('', [
		Validators.required,
		Validators.minLength(10),
		Validators.maxLength(500),
	]);

	protected selectPreset(reason: string) {
		this.selectedPreset.set(reason);
		this.showCustomReason.set(false);
	}

	protected selectOther() {
		this.selectedPreset.set(this.other);
		this.showCustomReason.set(true);
		this.reasonControl.setValue('');
		this.reasonControl.markAsTouched();
	}

	protected canSubmit(): boolean {
		if (!this.selectedPreset()) return false;
		if (this.selectedPreset() === this.other) return this.reasonControl.valid;
		return true;
	}

	protected onSubmit() {
		if (!this.canSubmit() || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		const reason = this.selectedPreset() === this.other
			? this.reasonControl.value!.trim()
			: this.selectedPreset()!;

		this.reportService.report({
			targetType: this.data.targetType,
			targetId: this.data.targetId,
			reason,
		}).subscribe({
			next: () => {
				this.submitting.set(false);
				this.snackbarService.createSnackbar(
					$localize`:@@report.submitted:Report submitted. Thank you for helping keep Playlog safe.`
				);
				this.dialogRef.close(true);
			},
			error: (err) => {
				this.submitting.set(false);
				this.error.set(err as ApiError);
			},
		});
	}
}
