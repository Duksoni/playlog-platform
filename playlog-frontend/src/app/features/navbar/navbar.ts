import {Component, inject} from '@angular/core';
import {MatToolbar} from '@angular/material/toolbar';
import {MatIconButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {Location} from '@angular/common';
import {MatTooltip} from '@angular/material/tooltip';
import {Router, RouterLink, RouterLinkActive} from '@angular/router';
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
		RouterLinkActive,
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

	private topLevelDestinations = [
		'home',
		'games',
		'genres',
		'tags',
		'platforms',
		'publishers',
		'developers',
	];

	protected get topLevelDestination() {
		const segments = this.location.path().split('/').filter(segment => segment);
		return segments.length === 1 && this.topLevelDestinations.includes(segments[0]);
	}

	protected get gameEntitiesActive() {
		const segments = this.location.path().split('/').filter(segment => segment);
		return segments.length > 0 && ['genres', 'tags', 'platforms', 'publishers', 'developers'].includes(segments[0]);
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

	protected readonly Role = Role;
}
