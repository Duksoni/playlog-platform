import {Component, inject} from '@angular/core';
import {SessionService} from '../../../core/services/session.service';
import {LibrarySectionComponent} from '../../../shared/components/library-section/library-section.component';

@Component({
	selector: 'app-library-page',
	standalone: true,
	imports: [LibrarySectionComponent],
	templateUrl: './library.page.html',
	styleUrl: './library.page.css',
})
export class LibraryPage {
	protected sessionService = inject(SessionService);

	protected userId: () => string;

	constructor() {
		this.userId = () => this.sessionService.user().userId;
	}
}
