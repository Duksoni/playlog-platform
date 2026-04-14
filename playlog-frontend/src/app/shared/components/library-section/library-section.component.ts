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
import {forkJoin, map, of, switchMap, catchError} from 'rxjs';
import {LibraryStatusDialog} from '../../../features/library/library-status-dialog/library-status.dialog';
import {DialogService} from '../../services/dialog.service';
import {ReviewDialog} from '../../../features/reviews/review-dialog/review.dialog';
import {ReviewService} from '../../../features/reviews/review.service';
import {ReviewSimpleResponse} from '../../../features/reviews/review.dto';

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
	private reviewService = inject(ReviewService);

	profileUserId = input.required<string>();
	showHeader = input<boolean>(true);

	protected readonly statuses = Object.values(GameLibraryStatus);
	protected readonly labels = LIBRARY_STATUS_LABELS;
	protected readonly icons = LIBRARY_STATUS_ICONS;

	protected loading = signal(true);
	protected activeStatus = signal<GameLibraryStatus>(GameLibraryStatus.PLAYING);
	protected gamesByStatus = signal<Partial<Record<GameLibraryStatus, LibraryGameCard[]>>>({});
	protected existingReviews = signal<Map<number, ReviewSimpleResponse>>(new Map());

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
		this.existingReviews.set(new Map());

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
			}),
			switchMap((cards) => {
				// If viewing own library, load existing reviews for all games
				if (this.isOwnLibrary && cards.length > 0) {
					const userId = this.sessionService.user().userId;
					const reviewRequests = cards.map(card =>
						this.reviewService.getReviewForUserAndGame(userId, card.gameId).pipe(
							map(review => ({gameId: card.gameId, review})),
							// Handle 404 (no review) gracefully
							catchError(() => of({gameId: card.gameId, review: null}))
						)
					);
					return forkJoin(reviewRequests).pipe(
						map(results => {
							const reviewMap = new Map<number, ReviewSimpleResponse>();
							for (const {gameId, review} of results) {
								if (review) {
									reviewMap.set(gameId, review);
								}
							}
							this.existingReviews.set(reviewMap);
							return cards;
						})
					);
				} else {
					return of(cards);
				}
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
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			if (result) this.loadLibrary();
		});
	}

	protected canReviewStatus(status: GameLibraryStatus): boolean {
		return status === GameLibraryStatus.COMPLETED || status === GameLibraryStatus.DROPPED;
	}

	protected hasExistingReview(gameId: number): boolean {
		return this.existingReviews().has(gameId);
	}

	protected getExistingReview(gameId: number): ReviewSimpleResponse | undefined {
		return this.existingReviews().get(gameId);
	}

	protected openReviewDialog(card: LibraryGameCard) {
		this.dialogService.openDialog(ReviewDialog, {
			data: {
				gameId: card.gameId,
				gameName: card.name,
				existing: this.getExistingReview(card.gameId) ?? null,
			},
			width: '560px',
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			// Dialog handles its own completion
			if (result && result !== 'deleted') {
				this.loadLibrary(); // Reload to refresh review status if needed
			}
		});
	}
}
