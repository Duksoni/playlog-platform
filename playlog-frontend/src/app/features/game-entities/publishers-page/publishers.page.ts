import {ChangeDetectionStrategy, Component} from '@angular/core';
import {GameEntitiesListComponent} from '../game-entities-list/game-entities-list.component';

@Component({
	selector: 'app-publishers-page',
	standalone: true,
	imports: [GameEntitiesListComponent],
	template: `
		<app-game-entities-list
			entityType="publishers"
			entityLabel="Publisher"
			entityIcon="domain"
			i18n-entityLabel="@@publishers"/>
	`,
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PublishersPage {}
