import {HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from '@angular/common/http';
import {inject, Injectable} from '@angular/core';
import {catchError, Observable, switchMap, throwError} from 'rxjs';
import {SessionService} from '../services/session.service';


@Injectable()
export class AuthInterceptor implements HttpInterceptor {
	private sessionService = inject(SessionService);

	private ignoredRoutes = [
		'/auth/login',
		'/auth/register',
		'/auth/refresh',
		'/auth/logout'
	];

	intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
		if (
			this.ignoredRoutes.some((route) => req.url.includes(route))
		) return next.handle(req);

		const token = this.sessionService.accessToken();

		// If no token, just forward the request
		if (!token) {
			return next.handle(req);
		}

		// If the token is expired, refresh first, then retry the original request
		if (this.sessionService.isTokenExpired()) {
			console.log('Token expired, attempting to refresh');

			return this.sessionService.refreshToken().pipe(
				switchMap((newToken) => {
					const cloned = req.clone({
						setHeaders: {Authorization: `Bearer ${newToken}`},
					});

					return next.handle(cloned);
				}),
				catchError((err) => {
					// Error handling is already done in SessionService.refreshToken()
					// We just need to propagate it or handle it if needed
					return throwError(() => err);
				})
			);
		}

		// If the token is valid, attach it and continue
		const authReq = req.clone({
			setHeaders: {Authorization: `Bearer ${token}`},
		});
		return next.handle(authReq);
	}
}
