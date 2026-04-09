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
	public games = signal<GameCard[]>([]);
	public loading = signal(false);
	public submitting = signal(false);

	private http = inject(HttpClient);
	private readonly base = `${environment.apiUrl}/games`;

	filterGames(params: GameFilterParams) {
		this.loading.set(true);
		this.fetchGames(params).pipe(
			switchMap(games => this.attachCovers(games))
		).subscribe({
			next: (gameCards) => {
				this.games.set(gameCards);
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	/** Append next page — used for infinite scroll. */
	filterGamesAppend(params: GameFilterParams) {
		this.loading.set(true);
		this.fetchGames(params).pipe(
			switchMap(games => this.attachCovers(games))
		).subscribe({
			next: (gameCards) => {
				this.games.update(existing => [...existing, ...gameCards]);
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	private fetchGames(params: GameFilterParams): Observable<GameSimple[]> {
		if (params.onlyDrafts) {
			return this.getUnpublishedGames();
		}
		if (params.developerId) {
			return this.getGamesByDeveloper(params.developerId);
		}
		if (params.publisherId) {
			return this.getGamesByPublisher(params.publisherId, params.page ?? 1);
		}
		return this.http.get<GameSimple[]>(`${this.base}/filter`, {params: this.buildParams(params)});
	}

	getGamesByDeveloper(developerId: number) {
		return this.http.get<GameSimple[]>(`${this.base}/by-developer/${developerId}`);
	}

	getGamesByPublisher(publisherId: number, page: number) {
		const params = new HttpParams().set('page', page.toString());
		return this.http.get<GameSimple[]>(`${this.base}/by-publisher/${publisherId}`, {params});
	}

	getGame(id: number) {
		return this.http.get<GameSimple>(`${this.base}/${id}`);
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

	private attachCovers(games: GameSimple[]) {
		if (games.length === 0) return of([] as GameCard[]);
		const gameIds = games.map(game => game.id);
		return this.getGameCovers(gameIds).pipe(
			map(coversResponse => games.map(game => ({
				...game,
				cover: coversResponse.gameCovers[game.id] ?? null,
			})))
		);
	}

	getGameCovers(gameIds: number[]) {
		let params = new HttpParams();
		for (const gameId of gameIds) {
			params = params.append('gameIds', gameId);
		}
		return this.http.get<GetGameCoversResponse>(`${environment.apiUrl}/media/games/covers`, {params: params});
	}

	getGameDetails(id: number) {
		return this.http.get<GameDetails>(`${this.base}/${id}/details`);
	}

	getGameMedia(gameId: number) {
		return this.http.get<GameMediaResponse>(`${environment.apiUrl}/media/games/${gameId}`);
	}

	getUnpublishedGames() {
		return this.http.get<GameSimple[]>(`${this.base}/unpublished`);
	}

	createGame(body: CreateGameRequest) {
		this.submitting.set(true);
		return this.http.post<GameDetails>(`${this.base}`, body);
	}

	updateGame(id: number, body: UpdateGameRequest) {
		this.submitting.set(true);
		return this.http.put<GameDetails>(`${this.base}/${id}`, body);
	}

	publishGame(id: number, body: PublishUnpublishGameRequest) {
		return this.http.put<Game>(`${this.base}/${id}/publish`, body);
	}

	unpublishGame(id: number, body: PublishUnpublishGameRequest) {
		return this.http.put<Game>(`${this.base}/${id}/unpublish`, body);
	}

	deleteGame(id: number) {
		return this.http.delete<void>(`${this.base}/${id}`);
	}

	uploadGameMedia(gameId: number, formData: FormData) {
		return this.http.post<GameMediaResponse>(
			`${environment.apiUrl}/media/games/${gameId}/upload`,
			formData,
		);
	}

	deleteGameMedia(gameId: number) {
		return this.http.delete<void>(`${environment.apiUrl}/media/games/${gameId}`);
	}
}
