import {Component, inject} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {SessionService} from '../../../core/services/session.service';
import {LibrarySectionComponent} from '../../../shared/components/library-section/library-section.component';
import {UserDetails} from '../../users/user.dto';

@Component({
	selector: 'app-library.page',
	standalone: true,
	imports: [LibrarySectionComponent],
	template: `<div class="library-page"><app-library-section [profileUserId]="userId()" [showHeader]="true" /></div>`,
	styles: [`
		.library-page {
			max-width: 900px;
			margin: 0 auto;
			padding: 32px 24px;
		}

		@media (max-width: 600px) {
			.library-page {
				padding: 20px 16px;
			}
		}
	`],
})
export class LibraryPage {
	private route = inject(ActivatedRoute);
	private router = inject(Router);
	protected sessionService = inject(SessionService);

	protected userId: () => string;

	constructor() {
		this.userId = () => this.sessionService.user().userId;
		this.validateAccess();
	}

	private validateAccess() {
		const navState = history.state as { userId?: string };
		const user = this.route.snapshot.data['user'] as UserDetails | null;

		// Only allow access to own library from /library route
		// Redirect to home if trying to access another user's library via this route
		if ((navState.userId && navState.userId !== this.sessionService.user().userId) || user) {
			this.router.navigate(['/home']);
		}
	}
}
