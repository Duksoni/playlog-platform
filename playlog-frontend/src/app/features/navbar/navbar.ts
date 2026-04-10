import {Component, inject} from '@angular/core';
import {MatToolbar} from '@angular/material/toolbar';
import {MatIconButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {Location} from '@angular/common';
import {MatTooltip} from '@angular/material/tooltip';
import {Router, RouterLink} from '@angular/router';
import {MatDivider} from '@angular/material/list';
import {MatMenu, MatMenuItem, MatMenuTrigger} from '@angular/material/menu';
import {SessionService} from '../../core/services/session.service';
import {Role} from '../auth/auth.dto';
import {HttpClient} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {DialogService} from '../../shared/services/dialog.service';
import {LoginDialog} from '../auth/login-dialog/login.dialog';
import {RegisterDialog} from '../auth/register-dialog/register.dialog';

@Component({
	selector: 'app-navbar',
	imports: [
		MatToolbar,
		MatIconButton,
		MatIcon,
		MatTooltip,
		RouterLink,
		MatDivider,
		MatMenuTrigger,
		MatMenu,
		MatMenuItem,
	],
	templateUrl: './navbar.html',
	styleUrl: './navbar.css',
})
export class Navbar {
	protected sessionService = inject(SessionService);
	private dialogService = inject(DialogService);

	protected location = inject(Location);
	private router = inject(Router);
	private http = inject(HttpClient);

	// Single-segment paths treated as top-level (no back button shown)
	private topLevelDestinations = [
		'home',
		'games',
		'library',
		'genres',
		'tags',
		'platforms',
		'publishers',
		'developers',
		'profile',
	];

	protected get topLevelDestination(): boolean {
		const segments = this.location.path().split('/').filter(s => s);
		if (segments.length === 1 && this.topLevelDestinations.includes(segments[0])) return true;
		return segments.length === 2 && segments[0] === 'users';
	}

	protected get gamesActive(): boolean {
		const segments = this.location.path().split('/').filter(s => s);
		if (segments.length === 0) return false;
		if (segments[0] === 'games') return true;
		// Matches /developers/:id/games and /publishers/:id/games
		return segments[segments.length - 1] === 'games';
	}

	protected get gameEntitiesActive(): boolean {
		const segments = this.location.path().split('/').filter(s => s);
		if (segments.length === 0) return false;
		const isEntityBase = ['genres', 'tags', 'platforms', 'publishers', 'developers'].includes(segments[0]);
		// Should not be highlighted if we are viewing games of that entity
		return isEntityBase && segments[segments.length - 1] !== 'games';
	}

	protected logout() {
		this.http.post(`${environment.apiUrl}/auth/logout`, {}, {withCredentials: true}).subscribe({
			next: () => {
				this.sessionService.handleLogout();
				this.router.navigate(['/home']);
			}
		});
	}

	protected changeTheme() {
		this.sessionService.toggleTheme();
	}

	protected openLoginDialog() {
		this.dialogService.openDialog(LoginDialog, {
			disableClose: true,
			autoFocus: false,
		});
	}

	protected openRegisterDialog() {
		this.dialogService.openDialog(RegisterDialog, {
			disableClose: true,
			autoFocus: false,
		});
	}

	protected navigateToLibrary() {
		this.router.navigate(['/library'], {
			state: {userId: this.sessionService.user().userId},
		});
	}

	protected readonly Role = Role;
}
