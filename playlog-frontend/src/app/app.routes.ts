import {Routes} from '@angular/router';
import {HomePage} from './features/home/home-page/home.page';
import {GenresPage} from './features/game-entities/genres-page/genres.page';
import {TagsPage} from './features/game-entities/tags-page/tags.page';
import {PlatformsPage} from './features/game-entities/platforms-page/platforms.page';
import {PublishersPage} from './features/game-entities/publishers-page/publishers.page';
import {DevelopersPage} from './features/game-entities/developers-page/developers.page';
import {GamesListPage} from './features/games/games-list-page/games-list.page';
import {GameDetailPage} from './features/games/game-detail-page/game-detail.page';
import {authGuard} from './core/guards/auth.guard';
import {Role} from './features/auth/auth.dto';

export const routes: Routes = [
	{
		path: 'home',
		component: HomePage,
	},
	{
		path: 'games',
		component: GamesListPage,
	},
	{
		path: 'developers/:id/games',
		component: GamesListPage,
	},
	{
		path: 'publishers/:id/games',
		component: GamesListPage,
	},
	{
		path: 'games/:id',
		component: GameDetailPage,
	},
	{
		path: 'genres',
		component: GenresPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
	},
	{
		path: 'tags',
		component: TagsPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
	},
	{
		path: 'platforms',
		component: PlatformsPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
	},
	{
		path: 'publishers',
		component: PublishersPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
	},
	{
		path: 'developers',
		component: DevelopersPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
	},
	{path: '', redirectTo: 'home', pathMatch: 'full'},
	{path: '**', redirectTo: 'home', pathMatch: 'full'},
];
