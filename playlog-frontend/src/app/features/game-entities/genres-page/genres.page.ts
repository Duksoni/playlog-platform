import {ChangeDetectionStrategy, Component} from '@angular/core';
import {GameEntitiesListComponent} from '../game-entities-list/game-entities-list.component';

@Component({
	selector: 'app-genres-page',
	standalone: true,
	imports: [GameEntitiesListComponent],
	template: `
		<app-game-entities-list entityType="genres" entityLabel="Genre" i18n-entityLabel="@@genresPage.label" [allowDelete]="true" />
	`,
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GenresPage {}
