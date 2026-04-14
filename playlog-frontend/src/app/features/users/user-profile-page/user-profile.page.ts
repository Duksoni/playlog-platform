import {Component, inject, signal} from '@angular/core';
import {DatePipe} from "@angular/common";
import {MatButton} from "@angular/material/button";
import {MatChip, MatChipSet} from "@angular/material/chips";
import {MatDivider} from "@angular/material/list";
import {MatIcon} from "@angular/material/icon";
import {MatProgressSpinner} from "@angular/material/progress-spinner";
import {ActivatedRoute, Router, RouterLink} from "@angular/router";
import {UserService} from '../user.service';
import {SessionService} from '../../../core/services/session.service';
import {UserDetails} from '../user.dto';
import {Role} from '../../auth/auth.dto';
import {LibrarySectionComponent} from '../../../shared/components/library-section/library-section.component';

@Component({
	selector: 'app-user-profile-page',
	imports: [
		DatePipe,
		MatButton,
		MatChip,
		MatChipSet,
		MatDivider,
		MatIcon,
		MatProgressSpinner,
		RouterLink,
		LibrarySectionComponent
	],
	templateUrl: './user-profile.page.html',
	styleUrl: './user-profile.page.css',
})
export class UserProfilePage {
	private route = inject(ActivatedRoute);
	private router = inject(Router);
	private userService = inject(UserService);
	protected sessionService = inject(SessionService);

	protected readonly Role = Role;

	protected user = signal<UserDetails | null>(null);
	protected loading = signal(true);

	protected get isOwnProfile(): boolean {
		return this.user()!.id === this.sessionService.user().userId;
	}

	ngOnInit() {
		const username = this.route.snapshot.paramMap.get('username');
		if (!username) {
			this.router.navigate(['/home']);
			return;
		}

		this.userService.getUser(username).subscribe({
			next: (user) => {
				this.user.set(user);
				this.loading.set(false);
			},
			error: () => {
				this.loading.set(false);
				this.router.navigate(['/home']);
			},
		});
	}

	protected getRoleLabel(role: string): string {
		switch (role) {
			case 'ADMIN':
				return $localize`:@@role.admin:Admin`;
			case 'MODERATOR':
				return $localize`:@@role.moderator:Moderator`;
			default:
				return $localize`:@@role.user:User`;
		}
	}
}
