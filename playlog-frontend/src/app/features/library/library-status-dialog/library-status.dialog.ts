import {Component, inject, signal} from '@angular/core';
import {MatButton} from "@angular/material/button";
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle
} from "@angular/material/dialog";
import {MatIcon} from "@angular/material/icon";
import {MatProgressSpinner} from "@angular/material/progress-spinner";
import {LibraryStatusDialogData} from './library-status-dialog-data';
import {LibraryService} from '../library.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ApiError} from '../../../core/api-error';
import {GameLibraryStatus, LIBRARY_STATUS_ICONS, LIBRARY_STATUS_LABELS, UserGame} from '../library.dto';

@Component({
	selector: 'app-library-status.dialog',
	imports: [
		MatButton,
		MatDialogActions,
		MatDialogClose,
		MatDialogContent,
		MatDialogTitle,
		MatIcon,
		MatProgressSpinner
	],
	templateUrl: './library-status.dialog.html',
	styleUrl: './library-status.dialog.css',
})
export class LibraryStatusDialog {
	protected data: LibraryStatusDialogData = inject(MAT_DIALOG_DATA);
	private dialogRef = inject(MatDialogRef<LibraryStatusDialog>);
	private libraryService = inject(LibraryService);
	private snackbarService = inject(SnackbarService);

	protected submitting = signal(false);
	protected error = signal<ApiError | null>(null);

	protected readonly statuses = Object.values(GameLibraryStatus);
	protected readonly labels = LIBRARY_STATUS_LABELS;
	protected readonly icons = LIBRARY_STATUS_ICONS;

	protected selectedStatus = signal<GameLibraryStatus | null>(this.data.currentStatus);

	protected selectStatus(status: GameLibraryStatus) {
		this.selectedStatus.set(status);
	}

	protected onSave() {
		const status = this.selectedStatus();
		if (!status || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		this.libraryService.addOrUpdate({gameId: this.data.gameId, status}).subscribe({
			next: (entry: UserGame) => {
				this.submitting.set(false);
				this.snackbarService.createSnackbar(
					$localize`:@@library.saved:${this.data.gameName} added to library.`
				);
				this.dialogRef.close(entry);
			},
			error: (err) => {
				this.submitting.set(false);
				this.error.set(err.error as ApiError);
			},
		});
	}

	protected onRemove() {
		if (this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		this.libraryService.remove(this.data.gameId).subscribe({
			next: () => {
				this.submitting.set(false);
				this.snackbarService.createSnackbar(
					$localize`:@@library.removed:${this.data.gameName} removed from library.`
				);
				this.dialogRef.close('removed');
			},
			error: (err) => {
				this.submitting.set(false);
				this.error.set(err.error as ApiError);
			},
		});
	}
}
