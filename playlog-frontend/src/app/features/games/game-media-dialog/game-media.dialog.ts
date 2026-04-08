import {ChangeDetectionStrategy, Component, inject, OnInit, signal} from '@angular/core';
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle,
} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatDividerModule} from '@angular/material/divider';
import {MatTooltipModule} from '@angular/material/tooltip';
import {GameService} from '../game.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ApiError} from '../../../core/api-error';
import {GameMediaResponse, MediaFileResponse} from '../game.dto';

interface PendingFile {
	field: 'cover' | 'screenshot' | 'trailer';
	file: File;
	previewUrl: string;
}

@Component({
	selector: 'app-game-media-dialog',
	imports: [
		MatDialogTitle,
		MatDialogContent,
		MatDialogActions,
		MatDialogClose,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatDividerModule,
		MatTooltipModule,
	],
	templateUrl: './game-media.dialog.html',
	styleUrl: './game-media.dialog.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameMediaDialog implements OnInit {
	protected data: GameMediaResponse = inject(MAT_DIALOG_DATA);
	private dialogRef = inject(MatDialogRef<GameMediaDialog>);
	private gameService = inject(GameService);
	private snackbarService = inject(SnackbarService);

	protected submitting = signal(false);
	protected error = signal<ApiError | null>(null);

	// Current saved media (shown as existing)
	protected currentCover = signal<MediaFileResponse | null>(null);
	protected currentScreenshots = signal<MediaFileResponse[]>([]);
	protected currentTrailer = signal<MediaFileResponse | null>(null);

	// Pending new files selected by user (not yet uploaded)
	protected pendingFiles = signal<PendingFile[]>([]);

	protected readonly MAX_IMAGE_MB = 10;
	protected readonly MAX_VIDEO_MB = 500;

	ngOnInit() {
		this.currentCover.set(this.data.cover ?? null);
		this.currentScreenshots.set(this.data.screenshots ?? []);
		this.currentTrailer.set(this.data.trailer ?? null);
	}

	// File selection helpers

	protected selectCover(event: Event) {
		const file = this.extractFile(event);
		if (!file || !this.validateImage(file)) return;
		this.replacePending('cover', file);
	}

	protected selectScreenshots(event: Event) {
		const input = event.target as HTMLInputElement;
		const files = Array.from(input.files ?? []);
		files.forEach(file => {
			if (this.validateImage(file)) {
				this.addPending('screenshot', file);
			}
		});
		input.value = '';
	}

	protected selectTrailer(event: Event) {
		const file = this.extractFile(event);
		if (!file || !this.validateVideo(file)) return;
		this.replacePending('trailer', file);
	}

	private extractFile(event: Event): File | null {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0] ?? null;
		input.value = '';
		return file;
	}

	private validateImage(file: File): boolean {
		if (!file.type.startsWith('image/')) {
			this.snackbarService.createSnackbar($localize`:@@media.invalidImage:File must be an image.`);
			return false;
		}
		if (file.size > this.MAX_IMAGE_MB * 1024 * 1024) {
			this.snackbarService.createSnackbar($localize`:@@media.imageTooLarge:Image must be under 10 MB.`);
			return false;
		}
		return true;
	}

	private validateVideo(file: File): boolean {
		if (!file.type.startsWith('video/')) {
			this.snackbarService.createSnackbar($localize`:@@media.invalidVideo:File must be a video.`);
			return false;
		}
		if (file.size > this.MAX_VIDEO_MB * 1024 * 1024) {
			this.snackbarService.createSnackbar($localize`:@@media.videoTooLarge:Video must be under 500 MB.`);
			return false;
		}
		return true;
	}

	private replacePending(field: 'cover' | 'trailer', file: File) {
		// Revoke old preview URL if any
		const existing = this.pendingFiles().find(p => p.field === field);
		if (existing) URL.revokeObjectURL(existing.previewUrl);

		const previewUrl = URL.createObjectURL(file);
		this.pendingFiles.update(list => [
			...list.filter(p => p.field !== field),
			{field, file, previewUrl},
		]);
	}

	private addPending(field: 'screenshot', file: File) {
		const previewUrl = URL.createObjectURL(file);
		this.pendingFiles.update(list => [...list, {field, file, previewUrl}]);
	}

	protected removePending(index: number) {
		this.pendingFiles.update(list => {
			URL.revokeObjectURL(list[index].previewUrl);
			return list.filter((_, i) => i !== index);
		});
	}

	protected pendingCover(): PendingFile | null {
		return this.pendingFiles().find(p => p.field === 'cover') ?? null;
	}

	protected pendingTrailer(): PendingFile | null {
		return this.pendingFiles().find(p => p.field === 'trailer') ?? null;
	}

	protected pendingScreenshots(): PendingFile[] {
		return this.pendingFiles().filter(p => p.field === 'screenshot');
	}

	// Upload

	protected get hasPendingChanges(): boolean {
		return this.pendingFiles().length > 0;
	}

	protected onUpload() {
		if (!this.hasPendingChanges || this.submitting()) return;
		this.submitting.set(true);
		this.error.set(null);

		const formData = new FormData();
		formData.append('version', this.data.version.toString());

		this.pendingFiles().forEach(p => {
			formData.append(p.field, p.file, p.file.name);
		});

		this.gameService.uploadGameMedia(this.data.gameId, formData).subscribe({
			next: (media) => {
				this.submitting.set(false);
				// Revoke all preview URLs
				this.pendingFiles().forEach(p => URL.revokeObjectURL(p.previewUrl));
				this.snackbarService.createSnackbar($localize`:@@media.uploadSuccess:Media uploaded successfully.`);
				this.dialogRef.close(media);
			},
			error: (err) => {
				this.submitting.set(false);
				this.error.set(err.error as ApiError);
			},
		});
	}
}
