import {inject, Injectable, signal} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {LoginRequest, TokenResponse} from '../auth.dto';
import {ApiError} from '../../../core/api-error';
import {SessionService} from '../../../core/services/session.service';
import {environment} from '../../../../environments/environment';

@Injectable({
	providedIn: 'root',
})
export class LoginService {
	public error = signal<ApiError | null>(null);
	public submitting = signal(false);

	private http = inject(HttpClient);
	private sessionService = inject(SessionService);

	login(data: LoginRequest) {
		this.submitting.set(true);
		this.error.set(null);
		this.http.post<TokenResponse>(`${environment.apiUrl}/auth/login`, data, {withCredentials: true}).subscribe({
			next: (response) => {
				this.sessionService.setAccessToken(response);
				this.error.set(null);
			},
			error: (err) => {
				const error = err as ApiError;
				this.error.set(error);
				this.submitting.set(false);
			}
		});
	}
}
