import {ApplicationConfig, inject, provideAppInitializer, provideBrowserGlobalErrorListeners} from '@angular/core';
import {provideRouter} from '@angular/router';

import {routes} from './app.routes';
import {HTTP_INTERCEPTORS, provideHttpClient, withInterceptorsFromDi} from '@angular/common/http';
import {AuthInterceptor} from './core/interceptors/auth.interceptor';
import {SessionService} from './core/services/session.service';
import {catchError, firstValueFrom, of} from 'rxjs';


export const appConfig: ApplicationConfig = {
	providers: [
		provideBrowserGlobalErrorListeners(),
		provideRouter(routes),
		provideHttpClient(withInterceptorsFromDi()),
		{
			provide: HTTP_INTERCEPTORS,
			useClass: AuthInterceptor,
			multi: true,
		},
		provideAppInitializer(initializeApp),
	]
};

function initializeApp() {
	const sessionService = inject(SessionService);
	if (sessionService.accessToken() && sessionService.isTokenExpired()) {
		return firstValueFrom(
			sessionService.refreshToken().pipe(
				catchError(() => of(null))
			)
		);
	}
	return Promise.resolve(null);
}
