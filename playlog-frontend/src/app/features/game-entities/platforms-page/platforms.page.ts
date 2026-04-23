import {ChangeDetectionStrategy, Component} from '@angular/core';
import {GameEntitiesListComponent} from '../game-entities-list/game-entities-list.component';

@Component({
	selector: 'app-platforms-page',
	standalone: true,
	imports: [GameEntitiesListComponent],
	template: `
		<app-game-entities-list
			entityType="platforms"
			entityLabel="Platform"
			entityIcon="desktop_windows"
			i18n-entityLabel="@@platformsPage.label"
			[allowDelete]="true"/>
	`,
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PlatformsPage {}
