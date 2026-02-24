import {computed, effect, inject, Injectable, signal} from '@angular/core';
import {JwtHelperService} from '@auth0/angular-jwt';
import {Role, TokenResponse, UserClaims} from '../../features/auth/auth.dto';
import {HttpBackend, HttpClient} from '@angular/common/http';
import {SnackbarService} from '../../shared/services/snackbar.service';
import {Router} from '@angular/router';
import {catchError, map, Observable, of, tap, throwError} from 'rxjs';
import {environment} from '../../../environments/environment';

@Injectable({
	providedIn: 'root',
})
export class SessionService {
	private helper = new JwtHelperService();
	private snackbarService = inject(SnackbarService);
	private router = inject(Router);
	private rawHttp = new HttpClient(inject(HttpBackend));

	readonly accessToken = signal<string | null>(localStorage.getItem('playlogAccessToken'));
	readonly theme = signal<'light' | 'dark'>(localStorage.getItem('theme') as 'light' | 'dark' || 'light');

	readonly user = computed<UserClaims>(() => {
		const token = this.accessToken();
		if (token) {
			try {
				const payload = this.helper.decodeToken(token);
				console.log("Token decoded, user claims updated.");
				return {
					userId: payload.sub,
					role: payload.role as Role,
					exp: new Date(payload.exp * 1000),
				};
			} catch {
				console.log("Error parsing payload, returning default user");
				return this.getDefaultUser();
			}
		} else {
			console.log("No token found, returning default user");
			return this.getDefaultUser();
		}
	});

	constructor() {
		// Persist tokens automatically
		effect(() => {
			const access = this.accessToken();
			if (access) localStorage.setItem('playlogAccessToken', access); else localStorage.removeItem('accessToken');
		});

		// Persist and apply the theme
		effect(() => {
			const theme = this.theme();
			localStorage.setItem('theme', theme);
			if (theme === 'dark') {
				document.documentElement.classList.add('dark-mode');
			} else {
				document.documentElement.classList.remove('dark-mode');
			}
		});
	}

	setAccessToken(response: TokenResponse) {
		this.accessToken.set(response.accessToken);
	}

	toggleTheme() {
		this.theme.update(t => t === 'light' ? 'dark' : 'light');
	}

	handleLogout() {
		this.accessToken.set(null);
		console.log("User has signed out, tokens cleared.");
	}

	refreshToken(): Observable<string | null> {
		const token = this.accessToken();
		if (!token) return of(null);

		return this.rawHttp
			.post<TokenResponse>(`${environment.apiUrl}/auth/refresh`, {}, {withCredentials: true})
			.pipe(
				tap((response) => {
					this.accessToken.set(response.accessToken);
				}),
				map(response => response.accessToken),
				catchError((err) => {
					if (err.status >= 500) {
						const serverErrorMessage = "Server is not responding. Please try again later."
						this.snackbarService.createSnackbar(serverErrorMessage);
					} else {
						// Refresh failed -> logout
						this.rawHttp
							.post(`${environment.apiUrl}/auth/logout`, {}, {withCredentials: true})
							.subscribe(() => {
								this.handleLogout();
								this.router.navigate(['/home']);
								this.snackbarService.createSnackbar('Your session has expired. Please sign in again.');
							});
					}
					return throwError(() => err);
				})
			);
	}

	isTokenExpired(): boolean {
		const token = this.accessToken();
		return token ? this.helper.isTokenExpired(token) : true;
	}

	private getDefaultUser(): UserClaims {
		return {
			userId: "",
			role: Role.GUEST,
		}
	}
}
