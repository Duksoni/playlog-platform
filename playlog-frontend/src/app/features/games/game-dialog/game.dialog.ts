import {ChangeDetectionStrategy, Component, inject, OnInit, signal} from '@angular/core';
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle,
} from '@angular/material/dialog';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatButtonModule} from '@angular/material/button';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatIconModule} from '@angular/material/icon';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {provideNativeDateAdapter} from '@angular/material/core';

import {GameDialogData} from './game-dialog-data';
import {GameService} from '../game.service';
import {ApiError} from '../../../core/api-error';
import {
	SearchableMultiSelectComponent
} from '../../../shared/components/searchable-multi-select/searchable-multi-select.component';

@Component({
	selector: 'app-game-dialog',
	imports: [
		ReactiveFormsModule,
		MatDialogTitle,
		MatDialogContent,
		MatDialogActions,
		MatDialogClose,
		MatFormFieldModule,
		MatInputModule,
		MatButtonModule,
		MatProgressSpinnerModule,
		MatIconModule,
		MatDatepickerModule,
		SearchableMultiSelectComponent,
	],
	providers: [provideNativeDateAdapter()],
	templateUrl: './game.dialog.html',
	styleUrl: './game.dialog.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameDialog implements OnInit {
	protected data: GameDialogData = inject(MAT_DIALOG_DATA);
	private dialogRef = inject(MatDialogRef<GameDialog>);
	private gameService = inject(GameService);

	protected submitting = signal(false);
	protected error = signal<ApiError | null>(null);

	protected get isEditing(): boolean {
		return !!this.data.existing;
	}

	protected gameForm = new FormGroup({
		name: new FormControl('', {
			nonNullable: true,
			validators: [Validators.required, Validators.minLength(1), Validators.maxLength(200)],
		}),
		description: new FormControl('', {
			nonNullable: true,
			validators: [Validators.required, Validators.minLength(10), Validators.maxLength(10000)],
		}),
		released: new FormControl<Date | null>(null),
		website: new FormControl('', {
			nonNullable: true,
		}),
	});

	// Selected IDs for each entity — driven by SearchableMultiSelect outputs
	protected selectedDeveloperIds = signal<number[]>([]);
	protected selectedPublisherIds = signal<number[]>([]);
	protected selectedGenreIds = signal<number[]>([]);
	protected selectedPlatformIds = signal<number[]>([]);
	protected selectedTagIds = signal<number[]>([]);

	ngOnInit() {
		const existing = this.data.existing;

		this.gameForm.patchValue({
			name: existing?.name ?? '',
			description: existing?.description ?? '',
			released: existing?.released ? new Date(existing.released) : null,
			website: existing?.website ?? '',
		});

		// Fill existing selections for edit mode
		if (existing) {
			this.selectedDeveloperIds.set(existing.developers.map(developer => developer.id));
			this.selectedPublisherIds.set(existing.publishers.map(publisher => publisher.id));
			this.selectedGenreIds.set(existing.genres.map(genre => genre.id));
			this.selectedPlatformIds.set(existing.platforms.map(platform => platform.id));
			this.selectedTagIds.set(existing.tags.map(tag => tag.id));
		}
	}

	// Outputs from SearchableMultiSelectComponent
	protected onDevelopersChange(ids: number[]) {
		this.selectedDeveloperIds.set(ids);
	}

	protected onPublishersChange(ids: number[]) {
		this.selectedPublisherIds.set(ids);
	}

	protected onGenresChange(ids: number[]) {
		this.selectedGenreIds.set(ids);
	}

	protected onPlatformsChange(ids: number[]) {
		this.selectedPlatformIds.set(ids);
	}

	protected onTagsChange(ids: number[]) {
		this.selectedTagIds.set(ids);
	}

	protected get multiSelectsValid(): boolean {
		return (
			this.selectedDeveloperIds().length > 0 &&
			this.selectedPublisherIds().length > 0 &&
			this.selectedGenreIds().length > 0 &&
			this.selectedPlatformIds().length > 0 &&
			this.selectedTagIds().length > 0
		);
	}

	protected onSubmit() {
		if (this.gameForm.invalid || !this.multiSelectsValid || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		const values = this.gameForm.getRawValue();
		const released = values.released instanceof Date
			? `${values.released.getFullYear()}-${String(values.released.getMonth() + 1).padStart(2, '0')}-${String(values.released.getDate()).padStart(2, '0')}`
			: values.released || null;

		if (this.isEditing) {
			this.gameService.updateGame(this.data.existing!.id, {
				name: values.name.trim(),
				description: values.description.trim(),
				released,
				website: values.website || null,
				version: this.data.existing!.version,
				developers: this.selectedDeveloperIds(),
				publishers: this.selectedPublisherIds(),
				genres: this.selectedGenreIds(),
				platforms: this.selectedPlatformIds(),
				tags: this.selectedTagIds(),
			}).subscribe({
				next: (updated) => {
					this.submitting.set(false);
					this.dialogRef.close(updated);
				},
				error: (err) => {
					this.submitting.set(false);
					this.error.set(err.error as ApiError);
				},
			});
		} else {
			this.gameService.createGame({
				name: values.name.trim(),
				description: values.description.trim(),
				released,
				website: values.website || null,
				developers: this.selectedDeveloperIds(),
				publishers: this.selectedPublisherIds(),
				genres: this.selectedGenreIds(),
				platforms: this.selectedPlatformIds(),
				tags: this.selectedTagIds(),
			}).subscribe({
				next: (created) => {
					this.submitting.set(false);
					this.dialogRef.close(created);
				},
				error: (err) => {
					this.submitting.set(false);
					this.error.set(err.error as ApiError);
				},
			});
		}
	}
}
