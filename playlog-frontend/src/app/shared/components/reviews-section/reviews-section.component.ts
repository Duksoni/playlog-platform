import {
	ChangeDetectionStrategy,
	ChangeDetectorRef,
	Component,
	effect,
	ElementRef,
	inject,
	input,
	NgZone,
	OnDestroy,
	OnInit,
	signal,
	ViewChild,
} from '@angular/core';
import {DatePipe} from '@angular/common';
import {Router} from '@angular/router';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatDividerModule} from '@angular/material/divider';
import {MatSelectModule} from '@angular/material/select';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatTooltipModule} from '@angular/material/tooltip';
import {ReviewService} from '../../../features/reviews/review.service';
import {
	GameRatingStatsResponse,
	GameReviewResponse,
	Rating,
	RATING_ICONS,
	RATING_LABELS,
	RATING_ORDER,
	ReviewSimpleResponse,
} from '../../../features/reviews/review.dto';
import {SessionService} from '../../../core/services/session.service';
import {Role} from '../../../features/auth/auth.dto';
import {DialogService} from '../../services/dialog.service';
import {ReviewDialog} from '../../../features/reviews/review-dialog/review.dialog';
import {GameLibraryStatus} from '../../../features/library/library.dto';
import {LibraryService} from '../../../features/library/library.service';
import {CommentsSectionComponent} from '../comments-section/comments-section.component';
import {CommentTargetType} from '../../../features/comments/comment.dto';

@Component({
	selector: 'app-reviews-section',
	imports: [
		DatePipe,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatDividerModule,
		MatSelectModule,
		MatFormFieldModule,
		MatTooltipModule,
		CommentsSectionComponent,
	],
	templateUrl: './reviews-section.component.html',
	styleUrl: './reviews-section.component.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ReviewsSectionComponent implements OnInit, OnDestroy {
	@ViewChild('scrollSentinel') set sentinel(element: ElementRef<HTMLElement> | undefined) {
		if (element && this.observer) {
			this.observer.disconnect();
			this.observer.observe(element.nativeElement);
		}
	}

	gameId = input.required<number>();
	gameName = input.required<string>();
	libraryStatus = input<GameLibraryStatus | null>(null);

	private reviewService = inject(ReviewService);
	private libraryService = inject(LibraryService);
	protected sessionService = inject(SessionService);
	private dialogService = inject(DialogService);
	private router = inject(Router);
	private zone = inject(NgZone);
	private cd = inject(ChangeDetectorRef);

	protected readonly Role = Role;
	protected readonly CommentTargetType = CommentTargetType;
	protected readonly ratingOrder = RATING_ORDER;
	protected readonly ratingLabels = RATING_LABELS;
	protected readonly ratingIcons = RATING_ICONS;

	protected reviews = signal<GameReviewResponse[]>([]);
	protected loading = signal(false);
	protected ownReview = signal<ReviewSimpleResponse | null>(null);
	protected canReview = signal(false);
	protected totalReviewsExist = signal(false);
	protected ratingStats = signal<GameRatingStatsResponse | null>(null);

	protected selectedRatingFilter = signal<Rating | null>(null);

	// Tracks which review's comments thread is expanded
	protected expandedCommentReviewId = signal<string | null>(null);

	// Tracks which reviews have expandable text
	protected expandableReviews = signal<Set<string>>(new Set());
	protected expandedReviews = signal<Set<string>>(new Set());

	private page = 0;
	private hasMore = true;
	private observer: IntersectionObserver | null = null;
	private readonly pageSize = 10;
	private initialUnfilteredLoadDone = false;

	constructor() {
		effect(() => {
			this.selectedRatingFilter();
			this.resetAndLoad();
		});

		effect(() => {
			this.gameId();
			this.loadRatingStats();
		});

		effect(() => {
			const status = this.libraryStatus();
			const eligible = [GameLibraryStatus.COMPLETED, GameLibraryStatus.DROPPED];
			this.canReview.set(status !== null && eligible.includes(status));
		});
	}

	ngOnInit() {
		this.setupObserver();
		const userId = this.sessionService.user().userId;
		if (userId) {
			this.reviewService.getReviewForUserAndGame(userId, this.gameId()).subscribe({
				next: (review) => this.ownReview.set(review),
				error: () => this.ownReview.set(null),
			});
		}
	}

	ngOnDestroy() {
		this.observer?.disconnect();
	}

	private setupObserver() {
		this.zone.runOutsideAngular(() => {
			this.observer = new IntersectionObserver(
				(entries) => {
					if (entries[0].isIntersecting && !this.loading() && this.hasMore) {
						this.zone.run(() => this.loadNextPage());
					}
				},
				{rootMargin: '500px'},
			);
		});
	}

	private resetAndLoad() {
		this.page = 0;
		this.hasMore = true;
		this.reviews.set([]);
		this.expandedCommentReviewId.set(null);
		this.loadPage(true);
	}

	private loadPage(replace: boolean) {
		this.loading.set(true);
		const isFirstUnfiltered = this.page === 0 && !this.selectedRatingFilter() && !this.initialUnfilteredLoadDone;
		this.reviewService.getReviewsForGame(
			this.gameId(),
			this.page,
			this.selectedRatingFilter() ?? undefined
		).subscribe({
			next: (data) => {
				this.loading.set(false);
				if (isFirstUnfiltered) {
					this.initialUnfilteredLoadDone = true;
					this.totalReviewsExist.set(data.length > 0);
				}
				if (data.length < this.pageSize) this.hasMore = false;
				replace ? this.reviews.set(data) : this.reviews.update(prev => [...prev, ...data]);
				setTimeout(() => this.checkAllReviewsOverflow(), 50);
			},
			error: () => this.loading.set(false),
		});
	}

	private loadNextPage() {
		if (!this.hasMore || this.loading()) return;
		this.page++;
		this.loadPage(false);
	}

	public reloadReviewEligibility() {
		const userId = this.sessionService.user().userId;
		if (userId) this.checkLibrary(userId);
	}

	private checkLibrary(userId: string) {
		this.libraryService.getUserLibrary(userId).subscribe({
			next: (entries) => {
				const entry = entries.find(e => e.gameId === this.gameId());
				const eligible = [GameLibraryStatus.COMPLETED, GameLibraryStatus.DROPPED];
				this.canReview.set(!!entry && eligible.includes(entry.status));
			},
			error: () => this.canReview.set(false),
		});
	}

	private loadRatingStats() {
		this.reviewService.getRatingStatsForGame(this.gameId()).subscribe({
			next: (stats) => {
				this.ratingStats.set(stats);
			},
			error: () => this.ratingStats.set(null),
		});
	}

	protected setRatingFilter(rating: Rating | null) {
		this.selectedRatingFilter.set(rating);
	}

	protected toggleComments(reviewId: string) {
		this.expandedCommentReviewId.update(id => id === reviewId ? null : reviewId);
	}

	protected openReviewDialog() {
		this.dialogService.openDialog(ReviewDialog, {
			data: {gameId: this.gameId(), gameName: this.gameName(), existing: this.ownReview()},
			width: '560px',
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			if (!result) return;
			if (result === 'deleted') {
				this.ownReview.set(null);
			} else {
				this.ownReview.set({
					id: result.id, gameId: result.gameId, userId: result.userId,
					username: result.username, rating: result.rating,
					text: result.text, createdAt: result.createdAt, updatedAt: result.updatedAt,
				});
			}
			this.resetAndLoad();
		});
	}

	protected navigateToUser(username: string) {
		this.router.navigate(['/users', username]);
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

	protected isOwnReviewCard(review: GameReviewResponse): boolean {
		return review.userId === this.sessionService.user().userId;
	}

	protected isReviewExpandable(reviewId: string): boolean {
		return this.expandableReviews().has(reviewId);
	}

	protected isReviewExpanded(reviewId: string): boolean {
		return this.expandedReviews().has(reviewId);
	}

	protected toggleReviewText(reviewId: string) {
		this.expandedReviews.update(set => {
			const newSet = new Set(set);
			if (newSet.has(reviewId)) {
				newSet.delete(reviewId);
			} else {
				newSet.add(reviewId);
			}
			return newSet;
		});
	}

	private checkAllReviewsOverflow() {
		const newExpandable = new Set<string>();
		this.reviews().forEach(review => {
			if (review.text) {
				const element = document.getElementById(`review-text-${review.id}`);
				if (element && element.scrollHeight > 150) {
					newExpandable.add(review.id);
				}
			}
		});
		this.expandableReviews.set(newExpandable);
		this.cd.markForCheck();
	}
}
