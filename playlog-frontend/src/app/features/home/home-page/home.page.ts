import {ChangeDetectionStrategy, Component, CUSTOM_ELEMENTS_SCHEMA, inject, OnInit, signal} from '@angular/core';
import {Router} from '@angular/router';
import {DatePipe, NgOptimizedImage, SlicePipe} from '@angular/common';
import {forkJoin, map, of, switchMap} from 'rxjs';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatTooltipModule} from '@angular/material/tooltip';
import {MatDividerModule} from '@angular/material/divider';
import {MatChipsModule} from '@angular/material/chips';
import {HomeService} from '../home.service';
import {GameCard, GameSimple} from '../../games/game.dto';
import {GameService} from '../../games/game.service';
import {LibraryService} from '../../library/library.service';
import {GameLibraryStatus, LibraryGameCard} from '../../library/library.dto';
import {SessionService} from '../../../core/services/session.service';
import {Role} from '../../auth/auth.dto';
import {RATING_ICONS, RATING_LABELS, Rating} from '../../reviews/review.dto';
import {RecentlyCommentedGame, RecentlyReviewedGame} from '../home.dto';
import {register} from 'swiper/element/bundle';

register();

@Component({
	selector: 'app-home-page',
	imports: [
		DatePipe,
		SlicePipe,
		NgOptimizedImage,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		MatDividerModule,
		MatChipsModule,
	],
	templateUrl: './home.page.html',
	styleUrl: './home.page.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
	schemas: [CUSTOM_ELEMENTS_SCHEMA],
})
export class HomePage implements OnInit {
	private homeService = inject(HomeService);
	private gameService = inject(GameService);
	private libraryService = inject(LibraryService);
	protected sessionService = inject(SessionService);
	private router = inject(Router);

	protected readonly Role = Role;
	protected readonly Rating = Rating;
	protected readonly ratingLabels = RATING_LABELS;
	protected readonly ratingIcons = RATING_ICONS;

	// Loading flags
	protected loadingMain = signal(true);
	protected loadingLibrary = signal(true);

	// Shelf rows
	protected newReleases = signal<GameCard[]>([]);
	protected mostReviewed = signal<GameCard[]>([]);
	protected topRated = signal<GameCard[]>([]);

	// Activity feeds
	protected recentReviews = signal<RecentlyReviewedGame[]>([]);
	protected recentComments = signal<RecentlyCommentedGame[]>([]);

	// Personalized row (logged-in)
	protected currentlyPlaying = signal<LibraryGameCard[]>([]);

	ngOnInit() {
		this.loadMain();
		this.loadPersonalised();
	}

	protected getRatingColorClass(rating: Rating): string {
		switch (rating) {
			case Rating.HIGHLY_RECOMMENDED:
				return 'rating-highly';
			case Rating.GOOD:
				return 'rating-good';
			case Rating.OKAY:
				return 'rating-okay';
			case Rating.NOT_RECOMMENDED:
				return 'rating-not';
			default:
				return '';
		}
	}

	private loadMain() {
		forkJoin({
			newReleases: this.homeService.getNewReleases(),
			mostReviewed: this.homeService.getMostReviewed(),
			topRated: this.homeService.getTopRated(),
			recentReviews: this.homeService.getRecentReviews(),
			recentComments: this.homeService.getRecentComments(),
		}).pipe(
			switchMap(data => {
				// Collect all unique game IDs across all three shelves
				const allIds = [
					...data.newReleases,
					...data.mostReviewed,
					...data.topRated,
				].map(g => g.id);
				const uniqueIds = [...new Set(allIds)];

				if (uniqueIds.length === 0) return of({...data, covers: {} as Record<number, string | null>});

				return this.gameService.getGameCovers(uniqueIds).pipe(
					map(coversResp => ({...data, covers: coversResp.gameCovers}))
				);
			})
		).subscribe({
			next: ({newReleases, mostReviewed, topRated, recentReviews, recentComments, covers}) => {
				const enrich = (games: GameSimple[]): GameCard[] =>
					games.map(g => ({...g, cover: covers[g.id] ?? null}));

				this.newReleases.set(enrich(newReleases));
				this.mostReviewed.set(enrich(mostReviewed));
				this.topRated.set(enrich(topRated));
				this.recentReviews.set(recentReviews);
				this.recentComments.set(recentComments);

				this.loadingMain.set(false);
			},
			error: () => this.loadingMain.set(false),
		});
	}

	private loadPersonalised() {
		const userId = this.sessionService.user().userId;
		if (!userId) {
			this.loadingLibrary.set(false);
			return;
		}

		this.libraryService.getUserLibrary(userId, GameLibraryStatus.PLAYING).pipe(
			switchMap(entries => {
				if (entries.length === 0) return of([] as LibraryGameCard[]);
				return this.gameService.getGameCovers(entries.map(e => e.gameId)).pipe(
					switchMap(coversResp =>
						forkJoin(entries.map(e => this.gameService.getGame(e.gameId))).pipe(
							map(games => {
								const gameMap = Object.fromEntries(games.map(g => [g.id, g]));
								return entries.map(e => ({
									...e,
									cover: coversResp.gameCovers[e.gameId] ?? null,
									name: gameMap[e.gameId]?.name ?? `Game #${e.gameId}`,
									released: gameMap[e.gameId]?.released ?? null,
								}));
							})
						)
					)
				);
			})
		).subscribe({
			next: (cards) => {
				this.currentlyPlaying.set(cards);
				this.loadingLibrary.set(false);
			},
			error: () => this.loadingLibrary.set(false),
		});
	}

	protected navigateToGame(id: number) {
		this.router.navigate(['/games', id]);
	}

	protected navigateToUser(username: string) {
		this.router.navigate(['/users', username]);
	}

	protected navigateToLibrary() {
		this.router.navigate(['/library'], {
			state: {userId: this.sessionService.user().userId},
		});
	}

	protected navigateToCatalogue(queryParams?: Record<string, string>) {
		this.router.navigate(['/games'], {queryParams});
	}
}
