import {inject, Injectable} from '@angular/core';
import {GameService} from '../games/game.service';
import {ReviewService} from '../reviews/review.service';
import {CommentService} from '../comments/comment.service';
import {map, Observable, switchMap} from 'rxjs';
import {RecentlyCommentedGame, RecentlyReviewedGame} from './home.dto';

@Injectable({
	providedIn: 'root',
})
export class HomeService {

	private gameService = inject(GameService);
	private reviewService = inject(ReviewService);
	private commentService = inject(CommentService);

	getNewReleases(limit = 8) {
		return this.gameService.getNewReleases();
	}

	getMostReviewed(limit = 8) {
		return this.reviewService.getMostReviewedGames(limit).pipe(
			switchMap(games =>
				this.gameService.getByIds(games.map(game => game.gameId)))
		);
	}

	getTopRated(limit = 8) {
		return this.reviewService.getTopRatedGames(limit).pipe(
			switchMap(games =>
				this.gameService.getByIds(games.map(g => g.gameId)))
		)
	}

	getRecentReviews(limit = 6): Observable<RecentlyReviewedGame[]> {
		return this.reviewService.getRecent(limit).pipe(
			switchMap(reviews =>
				this.gameService.getByIds(reviews.map(review => review.gameId)).pipe(
					map(games => {
						const gameMap = new Map(games.map(game => [game.id, game]));
						return reviews
							.map(review => {
								const game = gameMap.get(review.gameId);
								return game
									? {...review, gameName: game.name}
									: null;
							})
							.filter((item): item is RecentlyReviewedGame => item !== null);
					})
				)
			)
		);
	}

	getRecentComments(limit = 6): Observable<RecentlyCommentedGame[]> {
		return this.commentService.getRecentComments(limit).pipe(
			switchMap(comments =>
				this.gameService.getByIds(comments.map(comment => comment.gameId)).pipe(
					map(games => {
						const gameMap = new Map(games.map(game => [game.id, game]));
						return comments
							.map(comment => {
								const game = gameMap.get(comment.gameId);
								return game
									? {...comment, gameName: game.name}
									: null;
							})
							.filter((item): item is RecentlyCommentedGame => item !== null);
					})
				)
			)
		);
	}
}
