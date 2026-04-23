import {ChangeDetectionStrategy, Component, inject, OnInit, signal} from '@angular/core';
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle,
} from '@angular/material/dialog';
import {FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatError, MatFormField, MatLabel} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatButton} from '@angular/material/button';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {MatIcon} from '@angular/material/icon';
import {GameEntityDialogData} from './game-entity-dialog-data';
import {GameEntityService} from '../game-entity.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ApiError} from '../../../core/api-error';

@Component({
	selector: 'app-game-entity-dialog',
	imports: [
		MatDialogTitle,
		MatDialogContent,
		MatDialogActions,
		MatDialogClose,
		ReactiveFormsModule,
		MatFormField,
		MatLabel,
		MatError,
		MatInput,
		MatButton,
		MatProgressSpinner,
		MatIcon,
	],
	providers: [GameEntityService],
	templateUrl: './game-entity.dialog.html',
	styleUrl: './game-entity.dialog.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameEntityDialog implements OnInit {
	protected data: GameEntityDialogData = inject(MAT_DIALOG_DATA);
	private dialogRef = inject(MatDialogRef<GameEntityDialog>);
	private fb = inject(FormBuilder);
	protected entityService = inject(GameEntityService);
	private snackbarService = inject(SnackbarService);

	protected submitting = signal(false);
	protected error = signal<ApiError | null>(null);

	protected get isEditing(): boolean {
		return !!this.data.existing;
	}

	protected form!: FormGroup;

	ngOnInit() {
		this.form = this.fb.group({
			name: [
				this.data.existing?.name ?? '',
				[Validators.required, Validators.minLength(2), Validators.maxLength(100)],
			],
		});
	}

	protected onSubmit() {
		if (this.form.invalid || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		const name: string = this.form.value.name.trim();

		if (this.isEditing) {
			this.entityService.update(this.data.entityType, this.data.existing!.id, {
				name,
				version: this.data.existing!.version,
			}).subscribe({
				next: (updated) => {
					this.submitting.set(false);
					this.snackbarService.createSnackbar(`${this.data.entityLabel} updated successfully.`);
					this.dialogRef.close(updated);
				},
				error: (err) => {
					this.submitting.set(false);
					this.error.set(err as ApiError);
				},
			});
		} else {
			this.entityService.create(this.data.entityType, {name}).subscribe({
				next: (created) => {
					this.submitting.set(false);
					this.snackbarService.createSnackbar(`${this.data.entityLabel} created successfully.`);
					this.dialogRef.close(created);
				},
				error: (err) => {
					this.submitting.set(false);
					this.error.set(err as ApiError);
				},
			});
		}
	}
}
