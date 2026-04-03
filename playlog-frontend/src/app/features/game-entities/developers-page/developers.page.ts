import {ChangeDetectionStrategy, Component} from '@angular/core';
import {GameEntitiesListComponent} from '../game-entities-list/game-entities-list.component';

@Component({
	selector: 'app-developers-page',
	standalone: true,
	imports: [GameEntitiesListComponent],
	template: `
		<app-game-entities-list entityType="developers" entityLabel="Developer" i18n-entityLabel="@@developers"/>
	`,
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DevelopersPage {}
