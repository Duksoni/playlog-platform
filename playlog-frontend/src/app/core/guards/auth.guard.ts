import {inject} from '@angular/core';
import {ActivatedRouteSnapshot, CanActivateFn, Router, RouterStateSnapshot} from '@angular/router';
import {SessionService} from '../services/session.service';
import {Role} from '../../features/auth/auth.dto';

export const authGuard: CanActivateFn = (route: ActivatedRouteSnapshot, _: RouterStateSnapshot) => {
	const sessionService = inject(SessionService);
	const router = inject(Router);

	const user = sessionService.user();

	const roles = route.data['roles'] as Role[];
	if (!roles || roles.length === 0) {
		return true;
	}

	if (!roles.includes(user.role)) {
		router.navigate(["home"]);
		return false;
	}

	return true;
};
