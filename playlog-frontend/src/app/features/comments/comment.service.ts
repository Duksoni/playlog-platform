import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {
	CommentTargetType,
	CreateCommentRequest,
	DetailedCommentResponse, RecentCommentResponse,
	SimpleCommentResponse,
	UpdateCommentRequest,
} from './comment.dto';

@Injectable({
	providedIn: 'root',
})
export class CommentService {
	private http = inject(HttpClient);
	private readonly base = `${environment.apiUrl}/comments`;

	getComments(targetType: CommentTargetType, targetId: string, page?: number) {
		let params = new HttpParams()
			.set('targetType', targetType)
			.set('targetId', targetId);
		if (page !== undefined) params = params.set('page', (page + 1).toString());
		return this.http.get<SimpleCommentResponse[]>(this.base, {params});
	}

	addComment(body: CreateCommentRequest) {
		return this.http.post<DetailedCommentResponse>(this.base, body);
	}

	getComment(id: string) {
		return this.http.get<DetailedCommentResponse>(`${this.base}/${id}`);
	}

	getOwnComment(id: string) {
		return this.http.get<DetailedCommentResponse>(`${this.base}/me/${id}`);
	}

	updateComment(id: string, body: UpdateCommentRequest) {
		return this.http.put<DetailedCommentResponse>(`${this.base}/${id}`, body);
	}

	deleteComment(id: string) {
		return this.http.delete<void>(`${this.base}/${id}`);
	}

	getRecentComments(limit = 6) {
		const params = new HttpParams().set('limit', limit);
		return this.http.get<RecentCommentResponse[]>(`${this.base}/games/recent`, {params});
	}
}
