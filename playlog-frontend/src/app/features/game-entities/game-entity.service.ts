import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {
	CreateGameEntityRequest,
	GameEntity,
	GameEntitySimple,
	GameEntityType,
	UpdateGameEntityRequest
} from './game-entity.dto';
import {PagedResponse} from '../../core/paged-response.dto';
import {map} from 'rxjs';

@Injectable({
	providedIn: 'root',
})
export class GameEntityService {
	private readonly GET_ALL_LIMIT = 20;
	private http = inject(HttpClient);

	loadAll(entityType: GameEntityType, page: number, limit: number) {
		let params = new HttpParams();
		params = params.set('page', page + 1);
		params = params.set('limit', limit);
		return this.http.get<PagedResponse<GameEntitySimple>>(`${environment.apiUrl}/${entityType}`, {params});
	}

	/** First 20 by name — for dropdowns and autocomplete initial load. */
	getAllForFilter(entityType: GameEntityType) {
		let params = new HttpParams().set('limit', this.GET_ALL_LIMIT);
		return this.http.get<PagedResponse<GameEntitySimple>>(`${environment.apiUrl}/${entityType}`, {params})
			.pipe(map(
				response => response.data
			));
	}

	/** Search by partial name — for autocomplete. */
	searchForFilter(entityType: GameEntityType, query: string) {
		const params = new HttpParams().set('q', query).set('limit', this.GET_ALL_LIMIT);
		return this.http.get<GameEntitySimple[]>(`${environment.apiUrl}/${entityType}/search`, {params});
	}

	getById(entityType: GameEntityType, id: number) {
		return this.http.get<GameEntity>(`${environment.apiUrl}/${entityType}/${id}`);
	}

	search(entityType: GameEntityType, query: string, limit: number) {
		const params = new HttpParams().set('q', query).set('limit', limit);
		return this.http.get<GameEntitySimple[]>(`${environment.apiUrl}/${entityType}/search`, {params});
	}

	create(entityType: GameEntityType, body: CreateGameEntityRequest) {
		return this.http.post<GameEntity>(`${environment.apiUrl}/${entityType}`, body);
	}

	update(entityType: GameEntityType, id: number, body: UpdateGameEntityRequest) {
		return this.http.put<GameEntity>(`${environment.apiUrl}/${entityType}/${id}`, body);
	}

	delete(entityType: GameEntityType, id: number) {
		return this.http.delete<void>(`${environment.apiUrl}/${entityType}/${id}`);
	}
}
