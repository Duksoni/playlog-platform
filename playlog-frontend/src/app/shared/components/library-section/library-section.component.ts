import {ChangeDetectionStrategy, Component, computed, inject, input, OnInit, signal} from '@angular/core';
import {DatePipe, SlicePipe} from "@angular/common";
import {MatCard, MatCardContent} from "@angular/material/card";
import {MatIcon} from "@angular/material/icon";
import {MatIconButton} from "@angular/material/button";
import {MatProgressSpinner} from "@angular/material/progress-spinner";
import {MatTooltip} from "@angular/material/tooltip";
import {MatPaginator, PageEvent} from "@angular/material/paginator";
import {Router} from '@angular/router';
import {LibraryService} from '../../../features/library/library.service';
import {GameService} from '../../../features/games/game.service';
import {SessionService} from '../../../core/services/session.service';
import {
	GameLibraryStatus,
	LIBRARY_STATUS_ICONS,
	LIBRARY_STATUS_LABELS,
	LibraryGameCard
} from '../../../features/library/library.dto';
import {catchError, forkJoin, map, of, switchMap} from 'rxjs';
import {LibraryStatusDialog} from '../../../features/library/library-status-dialog/library-status.dialog';
import {DialogService} from '../../services/dialog.service';
import {ReviewDialog} from '../../../features/reviews/review-dialog/review.dialog';
import {ReviewService} from '../../../features/reviews/review.service';
import {ReviewSimpleResponse} from '../../../features/reviews/review.dto';

interface LibraryPageInfo {
	currentPage: number;
	totalPages: number;
	totalItems: number;
	limit: number;
}

const DEFAULT_LIMIT = 10;

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
		MatPaginator,
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
	protected cardsByStatus = signal<Record<GameLibraryStatus, LibraryGameCard[]>>({
		[GameLibraryStatus.OWNED]: [],
		[GameLibraryStatus.PLAYING]: [],
		[GameLibraryStatus.WISHLIST]: [],
		[GameLibraryStatus.COMPLETED]: [],
		[GameLibraryStatus.DROPPED]: [],
	});
	protected pageInfoByStatus = signal<Record<GameLibraryStatus, LibraryPageInfo>>({
		[GameLibraryStatus.OWNED]: {currentPage: 1, totalPages: 0, totalItems: 0, limit: DEFAULT_LIMIT},
		[GameLibraryStatus.PLAYING]: {currentPage: 1, totalPages: 0, totalItems: 0, limit: DEFAULT_LIMIT},
		[GameLibraryStatus.WISHLIST]: {currentPage: 1, totalPages: 0, totalItems: 0, limit: DEFAULT_LIMIT},
		[GameLibraryStatus.COMPLETED]: {currentPage: 1, totalPages: 0, totalItems: 0, limit: DEFAULT_LIMIT},
		[GameLibraryStatus.DROPPED]: {currentPage: 1, totalPages: 0, totalItems: 0, limit: DEFAULT_LIMIT},
	});
	protected existingReviews = signal<Map<number, ReviewSimpleResponse>>(new Map());

	protected get isOwnLibrary(): boolean {
		return this.profileUserId() === this.sessionService.user().userId;
	}

	protected activeGames = computed<LibraryGameCard[]>(() =>
		this.cardsByStatus()[this.activeStatus()] ?? []
	);

	protected activePageInfo = computed<LibraryPageInfo>(() =>
		this.pageInfoByStatus()[this.activeStatus()]
	);

	protected tabCount = (status: GameLibraryStatus): number =>
		this.pageInfoByStatus()[status]?.totalItems ?? 0;

	ngOnInit() {
		this.loadLibrary();
	}

	private loadLibrary() {
		this.loading.set(true);
		this.existingReviews.set(new Map());

		const userId = this.profileUserId();
		forkJoin(
			this.statuses.map(status =>
				this.libraryService.getUserLibrary(userId, status, 1, 1).pipe(
					map(response => ({status, totalItems: response.totalItems})),
					catchError(() => of({status, totalItems: 0}))
				)
			)
		).subscribe({
			next: (counts) => {
				const info = {...this.pageInfoByStatus()};
				for (const {status, totalItems} of counts) {
					info[status] = {
						...info[status],
						totalItems,
						totalPages: Math.ceil(totalItems / info[status].limit),
						currentPage: 1
					};
				}
				this.pageInfoByStatus.set(info);
				this.loadStatusPage(this.activeStatus(), 1);
			},
			error: () => this.loading.set(false),
		});
	}

	private loadStatusPage(status: GameLibraryStatus, page: number, limit?: number) {
		this.loading.set(true);
		const effectiveLimit = limit ?? this.pageInfoByStatus()[status]?.limit ?? DEFAULT_LIMIT;

		this.libraryService.getUserLibrary(this.profileUserId(), status, page, effectiveLimit).pipe(
			switchMap((response) => {
				const entries = response.data;
				const info = {
					currentPage: response.currentPage,
					totalPages: response.totalPages,
					totalItems: response.totalItems,
					limit: response.limit,
				};
				this.pageInfoByStatus.set({...this.pageInfoByStatus(), [status]: info});
				if (entries.length === 0) {
					return of([] as LibraryGameCard[]);
				}

				const gameIds = entries.map(e => e.gameId);

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
				if (this.isOwnLibrary && cards.length > 0) {
					const userId = this.sessionService.user().userId;
					const reviewRequests = cards.map(card =>
						this.reviewService.getReviewForUserAndGame(userId, card.gameId).pipe(
							map(review => ({gameId: card.gameId, review})),
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
				this.cardsByStatus.set({...this.cardsByStatus(), [status]: cards});
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	protected setStatus(status: GameLibraryStatus) {
		this.activeStatus.set(status);
		if ((this.cardsByStatus()[status] ?? []).length === 0 && this.tabCount(status) > 0) {
			this.loadStatusPage(status, 1);
		}
	}

	protected handlePageEvent(event: PageEvent) {
		const status = this.activeStatus();
		const currentLimit = this.pageInfoByStatus()[status]?.limit ?? DEFAULT_LIMIT;
		if (event.pageSize !== currentLimit) {
			this.loadStatusPage(status, 1, event.pageSize);
		} else {
			this.loadStatusPage(status, event.pageIndex + 1);
		}
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
			if (result === 'deleted') {
				this.existingReviews.update(reviews => {
					const next = new Map(reviews);
					next.delete(card.gameId);
					return next;
				});
			} else if (result) {
				this.loadLibrary();
			}
		});
	}
}
