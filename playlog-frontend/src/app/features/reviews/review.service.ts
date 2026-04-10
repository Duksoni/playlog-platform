import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {
	CreateUpdateReviewRequest,
	GameReviewResponse, GameRatingStatsResponse,
	Rating,
	ReviewDetailedResponse,
	ReviewSimpleResponse,
} from './review.dto';

@Injectable({
	providedIn: 'root',
})
export class ReviewService {
	private http = inject(HttpClient);
	private readonly base = `${environment.apiUrl}/reviews`;

	getReviewsForGame(gameId: number, page?: number, rating?: Rating) {
		let params = new HttpParams();
		if (page !== undefined) params = params.set('page', (page + 1).toString());
		if (rating) params = params.set('rating', rating);
		return this.http.get<GameReviewResponse[]>(`${this.base}/game/${gameId}`, {params});
	}

	getRatingStatsForGame(gameId: number) {
		return this.http.get<GameRatingStatsResponse>(`${this.base}/game/${gameId}/stats`);
	}

	getReviewForUserAndGame(userId: string, gameId: number) {
		return this.http.get<ReviewSimpleResponse>(`${this.base}/user/${userId}/game/${gameId}`);
	}

	getReview(id: string) {
		return this.http.get<ReviewDetailedResponse>(`${this.base}/${id}`);
	}

	upsertReview(body: CreateUpdateReviewRequest) {
		return this.http.post<ReviewDetailedResponse>(`${this.base}`, body);
	}

	deleteReview(id: string) {
		return this.http.delete<void>(`${this.base}/${id}`);
	}
}
