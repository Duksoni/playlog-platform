import {inject, Injectable, signal} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {
	CreateGameEntityRequest,
	GameEntity,
	GameEntitySimple,
	GameEntityType,
	UpdateGameEntityRequest
} from './game-entity.dto';
import {ApiError} from '../../core/api-error';

@Injectable({
	providedIn: 'root',
})
export class GameEntityService {
	public items = signal<GameEntitySimple[]>([]);
	public loading = signal(false);
	public submitting = signal(false);
	public error = signal<ApiError | null>(null);

	private http = inject(HttpClient);

	loadAll(entityType: GameEntityType, page?: number) {
		this.loading.set(true);
		let params = new HttpParams();
		if (page !== undefined) {
			// API uses 1-based indexing
			params = params.set('page', (page + 1).toString());
		}
		this.http.get<GameEntitySimple[]>(`${environment.apiUrl}/${entityType}`, {params}).subscribe({
			next: (data) => {
				this.items.set(data);
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	getById(entityType: GameEntityType, id: number) {
		return this.http.get<GameEntity>(`${environment.apiUrl}/${entityType}/${id}`);
	}

	search(entityType: GameEntityType, query: string) {
		this.loading.set(true);
		const params = new HttpParams().set('q', query);
		this.http.get<GameEntitySimple[]>(`${environment.apiUrl}/${entityType}/search`, {params}).subscribe({
			next: (data) => {
				this.items.set(data);
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	create(entityType: GameEntityType, body: CreateGameEntityRequest) {
		this.submitting.set(true);
		this.error.set(null);
		return this.http.post<GameEntity>(`${environment.apiUrl}/${entityType}`, body);
	}

	update(entityType: GameEntityType, id: number, body: UpdateGameEntityRequest) {
		this.submitting.set(true);
		this.error.set(null);
		return this.http.put<GameEntity>(`${environment.apiUrl}/${entityType}/${id}`, body);
	}

	delete(entityType: GameEntityType, id: number) {
		this.submitting.set(true);
		this.error.set(null);
		return this.http.delete<void>(`${environment.apiUrl}/${entityType}/${id}`);
	}
}

