import {inject, Injectable, signal} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {
	CreateGameRequest,
	Game,
	GameCard,
	GameDetails,
	GameFilterParams,
	GameMediaResponse,
	GameSimple,
	GetGameCoversResponse,
	PublishUnpublishGameRequest,
	UpdateGameRequest,
} from './game.dto';
import {map, Observable, of, switchMap} from 'rxjs';

@Injectable({
	providedIn: 'root',
})
export class GameService {
	private http = inject(HttpClient);
	private readonly gamesBase = `${environment.apiUrl}/games`;
	private readonly mediaBase = `${environment.apiUrl}/media/games`;

	getGamesByFilter(params: GameFilterParams): Observable<GameSimple[]> {
		if (params.onlyDrafts) {
			return this.getUnpublishedGames();
		}
		if (params.developerId) {
			return this.getGamesByDeveloper(params.developerId);
		}
		if (params.publisherId) {
			return this.getGamesByPublisher(params.publisherId, params.page ?? 1);
		}
		return this.http.get<GameSimple[]>(`${this.gamesBase}/filter`, {params: this.buildParams(params)});
	}

	getGamesByDeveloper(developerId: number) {
		return this.http.get<GameSimple[]>(`${this.gamesBase}/by-developer/${developerId}`);
	}

	getGamesByPublisher(publisherId: number, page: number) {
		const params = new HttpParams().set('page', page.toString());
		return this.http.get<GameSimple[]>(`${this.gamesBase}/by-publisher/${publisherId}`, {params});
	}

	getGame(id: number) {
		return this.http.get<GameSimple>(`${this.gamesBase}/${id}`);
	}

	private buildParams(filterParams: GameFilterParams): HttpParams {
		let params = new HttpParams();
		if (filterParams.name) params = params.set('name', filterParams.name);
		if (filterParams.page) params = params.set('page', filterParams.page.toString());
		if (filterParams.sort) params = params.set('sort', filterParams.sort);
		if (filterParams.platforms?.length) {
			for (const platform of filterParams.platforms) {
				params = params.append('platforms', platform);
			}
		}
		if (filterParams.genres?.length) {
			for (const genre of filterParams.genres) {
				params = params.append('genres', genre);
			}
		}
		if (filterParams.tags?.length) {
			for (const tag of filterParams.tags) {
				params = params.append('tags', tag);
			}
		}
		if (filterParams.sortDirection) params = params.set('sortDirection', filterParams.sortDirection);
		return params;
	}

	getNewReleases(limit = 8) {
		const params = new HttpParams().set('limit', limit);
		return this.http.get<GameSimple[]>(`${this.gamesBase}/new-releases`, {params});
	}

	getByIds(gameIds: number[]) {
		let params = new HttpParams();
		for (const gameId of gameIds) {
			params = params.append('gameIds', gameId);
		}
		return this.http.get<GameSimple[]>(`${this.gamesBase}/by-ids`, {params: params});
	}

	getGameCovers(gameIds: number[]) {
		let params = new HttpParams();
		for (const gameId of gameIds) {
			params = params.append('gameIds', gameId);
		}
		return this.http.get<GetGameCoversResponse>(`${this.mediaBase}/covers`, {params: params});
	}

	getGameDetails(id: number) {
		return this.http.get<GameDetails>(`${this.gamesBase}/${id}/details`);
	}

	getGameMedia(gameId: number) {
		return this.http.get<GameMediaResponse>(`${this.mediaBase}/${gameId}`);
	}

	getUnpublishedGames() {
		return this.http.get<GameSimple[]>(`${this.gamesBase}/unpublished`);
	}

	createGame(body: CreateGameRequest) {
		return this.http.post<GameDetails>(`${this.gamesBase}`, body);
	}

	updateGame(id: number, body: UpdateGameRequest) {
		return this.http.put<GameDetails>(`${this.gamesBase}/${id}`, body);
	}

	publishGame(id: number, body: PublishUnpublishGameRequest) {
		return this.http.put<Game>(`${this.gamesBase}/${id}/publish`, body);
	}

	unpublishGame(id: number, body: PublishUnpublishGameRequest) {
		return this.http.put<Game>(`${this.gamesBase}/${id}/unpublish`, body);
	}

	deleteGame(id: number) {
		return this.http.delete<void>(`${this.gamesBase}/${id}`);
	}

	uploadGameMedia(gameId: number, formData: FormData) {
		return this.http.post<GameMediaResponse>(
			`${this.mediaBase}/${gameId}/upload`,
			formData,
		);
	}

	deleteGameMedia(gameId: number) {
		return this.http.delete<void>(`${this.mediaBase}/${gameId}`);
	}
}
