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

const PRESET_REASONS: string[] = [
	$localize`:@@report.reasonHate:Hate speech or harassment`,
	$localize`:@@report.reasonSpam:Spam or advertising`,
	$localize`:@@report.reasonSpoilers:Unmarked spoilers`,
	$localize`:@@report.reasonInappropriate:Inappropriate content`,
	$localize`:@@report.reasonOther:Other`,
];

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
	protected readonly ReportTargetType = ReportTargetType;

	protected submitting = signal(false);
	protected error = signal<ApiError | null>(null);
	protected selectedPreset = signal<string | null>(null);

	protected reasonControl = new FormControl('', [
		Validators.required,
		Validators.minLength(10),
		Validators.maxLength(500),
	]);

	constructor() {
		this.reasonControl.valueChanges.subscribe(val => {
			// If user types something that doesn't match any preset (excluding 'Other'), deselect
			const other = this.presetReasons[this.presetReasons.length - 1];
			if (val !== this.selectedPreset() && this.selectedPreset() !== other) {
				this.selectedPreset.set(null);
			}
		});
	}

	protected selectPreset(reason: string) {
		this.selectedPreset.set(reason);
		const other = this.presetReasons[this.presetReasons.length - 1];
		if (reason === other) {
			this.reasonControl.setValue('');
		} else {
			this.reasonControl.setValue(reason);
		}
		this.reasonControl.markAsTouched();
	}

	protected onSubmit() {
		if (this.reasonControl.invalid || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		this.reportService.report({
			targetType: this.data.targetType,
			targetId: this.data.targetId,
			reason: this.reasonControl.value!.trim(),
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
