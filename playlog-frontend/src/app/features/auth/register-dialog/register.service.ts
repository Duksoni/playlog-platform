import {inject, Injectable, signal} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {RegisterRequest} from '../auth.dto';
import {environment} from '../../../../environments/environment';

@Injectable({
	providedIn: 'root',
})
export class RegisterService {
	public error = signal<string | null>(null);
	public submitting = signal(false);
	public success = signal(false);

	private http = inject(HttpClient);

	register(data: RegisterRequest) {
		this.submitting.set(true);
		this.error.set(null);
		this.http.post(`${environment.apiUrl}/auth/register`, data).subscribe({
			next: () => {
				this.error.set(null);
				this.success.set(true);
			},
			error: (err) => {
				this.error.set(err.error?.message || 'Sign up failed');
				this.submitting.set(false);
			}
		});
	}
}
