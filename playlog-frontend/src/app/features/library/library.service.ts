import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {AddUpdateGameRequest, GameLibraryStatus, LibraryGame, UserGame} from './library.dto';
import {PagedResponse} from '../../core/paged-response.dto';

@Injectable({
	providedIn: 'root',
})
export class LibraryService {
	private http = inject(HttpClient);
	private readonly base = `${environment.apiUrl}/library`;

	getUserLibrary(userId: string, status?: GameLibraryStatus, page = 1, limit = 50) {
		let params = new HttpParams().set('page', page).set('limit', limit);
		if (status) params = params.set('status', status);
		return this.http.get<PagedResponse<LibraryGame>>(`${this.base}/user/${userId}`, {params});
	}

	addOrUpdate(body: AddUpdateGameRequest) {
		return this.http.post<UserGame>(`${this.base}`, body);
	}

	remove(gameId: number) {
		return this.http.delete<void>(`${this.base}/${gameId}`);
	}
}
