import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {AddUpdateGameRequest, GameLibraryStatus, LibraryGame, LibraryStats, UserGame} from './library.dto';

@Injectable({
	providedIn: 'root',
})
export class LibraryService {
	private http = inject(HttpClient);
	private readonly base = `${environment.apiUrl}/library`;

	getUserLibrary(userId: string, status?: GameLibraryStatus) {
		let params = new HttpParams();
		if (status) params = params.set('status', status);
		return this.http.get<LibraryGame[]>(`${this.base}/user/${userId}`, {params});
	}

	getLibraryStats(userId: string) {
		return this.http.get<LibraryStats>(`${this.base}/user/${userId}/stats`);
	}

	addOrUpdate(body: AddUpdateGameRequest) {
		return this.http.post<UserGame>(`${this.base}`, body);
	}

	remove(gameId: number) {
		return this.http.delete<void>(`${this.base}/${gameId}`);
	}
}
