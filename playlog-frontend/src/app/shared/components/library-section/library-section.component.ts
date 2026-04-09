import {ChangeDetectionStrategy, Component, computed, inject, input, OnInit, signal} from '@angular/core';
import {DatePipe, SlicePipe} from "@angular/common";
import {MatCard, MatCardContent} from "@angular/material/card";
import {MatIcon} from "@angular/material/icon";
import {MatIconButton} from "@angular/material/button";
import {MatProgressSpinner} from "@angular/material/progress-spinner";
import {MatTooltip} from "@angular/material/tooltip";
import {Router} from '@angular/router';
import {LibraryService} from '../../../features/library/library.service';
import {GameService} from '../../../features/games/game.service';
import {SessionService} from '../../../core/services/session.service';
import {
	GameLibraryStatus,
	LIBRARY_STATUS_ICONS,
	LIBRARY_STATUS_LABELS,
	LibraryGame,
	LibraryGameCard
} from '../../../features/library/library.dto';
import {forkJoin, map, of, switchMap} from 'rxjs';
import {LibraryStatusDialog} from '../../../features/library/library-status-dialog/library-status.dialog';
import {DialogService} from '../../services/dialog.service';

@Component({
	selector: 'app-library-section',
	standalone: true,
	imports: [
		DatePipe,
		MatCard,
		MatCardContent,
		MatIcon,
		MatIconButton,
		MatProgressSpinner,
		MatTooltip,
		SlicePipe
	],
	templateUrl: './library-section.component.html',
	styleUrl: './library-section.component.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class LibrarySectionComponent implements OnInit {
	private router = inject(Router);
	private libraryService = inject(LibraryService);
	private gameService = inject(GameService);
	protected sessionService = inject(SessionService);
	private dialogService = inject(DialogService);

	profileUserId = input.required<string>();
	showHeader = input<boolean>(true);

	protected readonly statuses = Object.values(GameLibraryStatus);
	protected readonly labels = LIBRARY_STATUS_LABELS;
	protected readonly icons = LIBRARY_STATUS_ICONS;

	protected loading = signal(true);
	protected activeStatus = signal<GameLibraryStatus>(GameLibraryStatus.PLAYING);
	protected gamesByStatus = signal<Partial<Record<GameLibraryStatus, LibraryGameCard[]>>>({});

	protected get isOwnLibrary(): boolean {
		return this.profileUserId() === this.sessionService.user().userId;
	}

	protected activeGames = computed<LibraryGameCard[]>(() =>
		this.gamesByStatus()[this.activeStatus()] ?? []
	);

	protected tabCount = (status: GameLibraryStatus): number =>
		this.gamesByStatus()[status]?.length ?? 0;

	ngOnInit() {
		this.loadLibrary();
	}

	private loadLibrary() {
		this.loading.set(true);

		this.libraryService.getUserLibrary(this.profileUserId()).pipe(
			switchMap((entries: LibraryGame[]) => {
				if (entries.length === 0) {
					return of([] as LibraryGameCard[]);
				}

				const gameIds = entries.map(e => e.gameId);

				// Fetch covers and basic game info in parallel
				return forkJoin([
					this.gameService.getGameCovers(gameIds),
					forkJoin(gameIds.map(id => this.gameService.getGame(id)))
				]).pipe(
					map(([coversResponse, gameInfos]) => {
						const infoMap = Object.fromEntries(gameInfos.map(game => [game.id, game]));
						return entries.map(libraryGame => ({
							...libraryGame,
							cover: coversResponse.gameCovers[libraryGame.gameId] ?? null,
							name: infoMap[libraryGame.gameId]?.name ?? `Game #${libraryGame.gameId}`,
							released: infoMap[libraryGame.gameId]?.released ?? null,
						}));
					})
				);
			})
		).subscribe({
			next: (cards) => {
				const grouped: Partial<Record<GameLibraryStatus, LibraryGameCard[]>> = {};
				for (const status of this.statuses) {
					grouped[status] = cards.filter(c => c.status === status);
				}
				this.gamesByStatus.set(grouped);
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	protected setStatus(status: GameLibraryStatus) {
		this.activeStatus.set(status);
	}

	protected navigateToGame(gameId: number) {
		this.router.navigate(['/games', gameId]);
	}

	protected openStatusDialog(card: LibraryGameCard) {
		this.dialogService.openDialog(LibraryStatusDialog, {
			data: {
				gameId: card.gameId,
				gameName: card.name,
				currentStatus: card.status,
			},
			width: '440px',
			disableClose: false,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			if (result) this.loadLibrary();
		});
	}
}
