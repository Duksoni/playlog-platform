import {
	AfterViewInit,
	ChangeDetectionStrategy,
	Component,
	ElementRef,
	inject,
	OnInit,
	signal,
	ViewChild
} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {DatePipe} from '@angular/common';
import {concatMap} from 'rxjs';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatChipsModule} from '@angular/material/chips';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatTooltipModule} from '@angular/material/tooltip';
import {MatDividerModule} from '@angular/material/divider';
import {MatTabsModule} from '@angular/material/tabs';
import {MatCardModule} from '@angular/material/card';
import {GameService} from '../game.service';
import {GameDetails, GameMediaResponse} from '../game.dto';
import {SessionService} from '../../../core/services/session.service';
import {Role} from '../../auth/auth.dto';
import {DialogService} from '../../../shared/services/dialog.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {GameDialog} from '../game-dialog/game.dialog';
import {GameMediaDialog} from '../game-media-dialog/game-media.dialog';
import {LibraryService} from '../../library/library.service';
import {GameLibraryStatus, LIBRARY_STATUS_ICONS, LIBRARY_STATUS_LABELS} from '../../library/library.dto';
import {LibraryStatusDialog} from '../../library/library-status-dialog/library-status.dialog';

@Component({
	selector: 'app-game-detail-page',
	imports: [
		DatePipe,
		MatButtonModule,
		MatIconModule,
		MatChipsModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		MatDividerModule,
		MatTabsModule,
		MatCardModule,
	],
	templateUrl: './game-detail.page.html',
	styleUrl: './game-detail.page.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameDetailPage implements OnInit, AfterViewInit {
	private route = inject(ActivatedRoute);
	private router = inject(Router);
	private gameService = inject(GameService);
	protected sessionService = inject(SessionService);
	private dialogService = inject(DialogService);
	private snackbarService = inject(SnackbarService);
	private libraryService = inject(LibraryService);

	protected readonly Role = Role;
	protected readonly statusLabels = LIBRARY_STATUS_LABELS;
	protected readonly statusIcons = LIBRARY_STATUS_ICONS;

	protected game = signal<GameDetails | null>(null);
	protected media = signal<GameMediaResponse | null>(null);
	protected loading = signal(true);
	protected mediaLoading = signal(true);
	protected selectedScreenshot = signal<string | null>(null);

	// Library status for the current logged-in user
	protected libraryStatus = signal<GameLibraryStatus | null>(null);

	@ViewChild('descriptionText') descriptionText?: ElementRef<HTMLParagraphElement>;
	protected isDescriptionExpandable = signal(false);
	protected isDescriptionExpanded = signal(false);

	ngOnInit() {
		const id = Number(this.route.snapshot.paramMap.get('id'));
		if (!id) {
			this.router.navigate(['/games']);
			return;
		}

		this.gameService.getGameDetails(id).subscribe({
			next: (game) => {
				this.game.set(game);
				this.loading.set(false);
				setTimeout(() => this.checkDescriptionOverflow(), 0);
			},
			error: () => {
				this.loading.set(false);
				this.router.navigate(['/games']);
			},
		});

		this.gameService.getGameMedia(id).subscribe({
			next: (media) => {
				this.media.set(media);
				if (media.screenshots.length > 0) {
					this.selectedScreenshot.set(media.screenshots[0].url);
				}
				this.mediaLoading.set(false);
			},
			error: (err) => {
				this.mediaLoading.set(false);
				if (err.status === 404) {
					this.media.set({gameId: id, screenshots: [], version: 0});
				}
			},
		});

		// Load library status for logged-in user
		const userId = this.sessionService.user().userId;
		if (userId) {
			this.libraryService.getUserLibrary(userId).subscribe({
				next: (entries) => {
					const entry = entries.find(e => e.gameId === id);
					this.libraryStatus.set(entry?.status ?? null);
				},
				error: () => this.libraryStatus.set(null),
			});
		}
	}

	ngAfterViewInit() {
		this.checkDescriptionOverflow();
	}

	private checkDescriptionOverflow() {
		if (this.descriptionText) {
			const element = this.descriptionText.nativeElement;
			this.isDescriptionExpandable.set(element.scrollHeight > 300);
		}
	}

	protected toggleDescription() {
		this.isDescriptionExpanded.update(v => !v);
	}

	protected openLibraryDialog() {
		const game = this.game();
		if (!game) return;

		this.dialogService.openDialog(LibraryStatusDialog, {
			data: {
				gameId: game.id,
				gameName: game.name,
				currentStatus: this.libraryStatus(),
			},
			width: '440px',
			disableClose: false,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			if (result === 'removed') {
				this.libraryStatus.set(null);
			} else if (result?.status) {
				this.libraryStatus.set(result.status);
			}
		});
	}

	protected openEditDialog() {
		const game = this.game();
		if (!game) return;

		this.dialogService.openDialog(GameDialog, {
			data: {existing: game},
			width: '1000px',
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(updated => {
			if (updated) {
				this.game.set(updated);
				this.snackbarService.createSnackbar($localize`:@@games.updated:Game updated successfully.`);
				setTimeout(() => this.checkDescriptionOverflow(), 0);
			}
		});
	}

	protected openMediaDialog() {
		const game = this.game();
		const media = this.media();
		if (!game || !media) return;

		this.dialogService.openDialog(GameMediaDialog, {
			data: media,
			width: '600px',
			maxWidth: '50vw',
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(updatedMedia => {
			if (updatedMedia) {
				this.media.set(updatedMedia);
				if (updatedMedia.screenshots.length > 0) {
					this.selectedScreenshot.set(updatedMedia.screenshots[0].url);
				} else {
					this.selectedScreenshot.set(null);
				}
			}
		});
	}

	protected togglePublish() {
		const game = this.game();
		if (!game) return;

		const isPublishing = game.draft;
		const dialogRef = this.dialogService.openSimpleDialog({
			width: '420px',
			data: {
				title: isPublishing
					? $localize`:@@gameDetail.publishTitle:Publish Game`
					: $localize`:@@gameDetail.unpublishTitle:Unpublish Game`,
				content: isPublishing
					? $localize`:@@gameDetail.publishContent:Are you sure you want to publish "${game.name}"? It will be visible to all users.`
					: $localize`:@@gameDetail.unpublishContent:Are you sure you want to move "${game.name}" back to draft? It will be hidden from regular users.`,
			},
		});

		dialogRef.componentInstance.setPositiveButton(
			isPublishing
				? $localize`:@@gameDetail.publish:Publish`
				: $localize`:@@gameDetail.unpublish:Unpublish`,
			() => {
				const action = isPublishing
					? this.gameService.publishGame(game.id, {version: game.version})
					: this.gameService.unpublishGame(game.id, {version: game.version});

				action.subscribe({
					next: (updated) => {
						this.game.update(g => g ? {...g, draft: updated.draft, version: updated.version} : g);
						const msg = updated.draft
							? $localize`:@@games.unpublished:Game moved back to draft.`
							: $localize`:@@games.published:Game published successfully.`;
						this.snackbarService.createSnackbar(msg);
						dialogRef.close();
					},
					error: (err) => {
						if (err.status === 409) {
							this.snackbarService.createSnackbar($localize`:@@games.versionConflict:This game was modified by someone else. Please refresh.`);
						}
						dialogRef.close();
					},
				});
			},
		);
		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`);
	}

	protected confirmDelete() {
		const game = this.game();
		if (!game) return;

		const dialogRef = this.dialogService.openSimpleDialog({
			width: '420px',
			data: {
				title: $localize`:@@gameDetail.deleteTitle:Delete Game`,
				content: $localize`:@@gameDetail.deleteContent:Are you sure you want to delete "${game.name}"? This cannot be undone.`,
			},
		});

		dialogRef.componentInstance.setPositiveButton(
			$localize`:@@common.delete:Delete`,
			() => {
				this.gameService.deleteGameMedia(game.id).pipe(
					concatMap(() => this.gameService.deleteGame(game.id))
				).subscribe({
					next: () => {
						this.snackbarService.createSnackbar($localize`:@@games.deleted:Game deleted successfully.`);
						this.router.navigate(['/games']);
						dialogRef.close();
					},
					error: () => {
						this.snackbarService.createSnackbar($localize`:@@games.deleteFailed:Failed to delete game.`);
						dialogRef.close();
					},
				});
			},
		);
		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`);
	}

	protected selectScreenshot(url: string) {
		this.selectedScreenshot.set(url);
	}

	protected navigateToGamesByEntity(type: 'developers' | 'publishers', id: number, name: string) {
		this.router.navigate([`/${type}`, id, 'games'], {state: {name}});
	}
}
