import {ChangeDetectionStrategy, Component} from '@angular/core';
import {GameEntitiesListComponent} from '../game-entities-list/game-entities-list.component';

@Component({
	selector: 'app-tags-page',
	standalone: true,
	imports: [GameEntitiesListComponent],
	template: `
		<app-game-entities-list entityType="tags" entityLabel="Tag" i18n-entityLabel="@@tagsPage.label" [allowDelete]="true" />
	`,
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class TagsPage {}
