import {Routes} from '@angular/router';
import {HomePage} from './features/home/home-page/home.page';
import {GenresPage} from './features/game-entities/genres-page/genres.page';
import {TagsPage} from './features/game-entities/tags-page/tags.page';
import {PlatformsPage} from './features/game-entities/platforms-page/platforms.page';
import {PublishersPage} from './features/game-entities/publishers-page/publishers.page';
import {DevelopersPage} from './features/game-entities/developers-page/developers.page';
import {GamesListPage} from './features/games/games-list-page/games-list.page';
import {GameDetailPage} from './features/games/game-detail-page/game-detail.page';
import {UserProfilePage} from './features/users/user-profile-page/user-profile.page';
import {MyProfilePage} from './features/users/my-profile-page/my-profile.page';
import {AdminUsersPage} from './features/users/admin-users-page/admin-users.page';
import {LibraryPage} from './features/library/library-page/library.page';
import {ReportsPage} from './features/reports/reports-page/reports.page';
import {authGuard} from './core/guards/auth.guard';
import {Role} from './features/auth/auth.dto';

const titlePrefix = 'Playlog | ';
const withTitlePrefix = (title: string) => `${titlePrefix}${title}`;

export const routes: Routes = [
	{
		path: 'home',
		component: HomePage,
		title: withTitlePrefix($localize`:@@routes.home:Home`),
	},
	{
		path: 'games',
		component: GamesListPage,
		title: withTitlePrefix($localize`:@@routes.games:Games`),
	},
	{
		path: 'developers/:id/games',
		component: GamesListPage,
		title: withTitlePrefix($localize`:@@routes.developerGames:Developer Games`),
	},
	{
		path: 'publishers/:id/games',
		component: GamesListPage,
		title: withTitlePrefix($localize`:@@routes.publisherGames:Publisher Games`),
	},
	{
		path: 'games/:id',
		component: GameDetailPage,
		title: withTitlePrefix($localize`:@@routes.gameDetails:Game Details`),
	},
	{
		path: 'profile',
		component: MyProfilePage,
		canActivate: [authGuard],
		data: {roles: [Role.USER, Role.MODERATOR, Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.myProfile:My Profile`),
	},
	{
		path: 'library',
		component: LibraryPage,
		canActivate: [authGuard],
		data: {roles: [Role.USER, Role.MODERATOR, Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.myLibrary:My Library`),
	},
	{
		path: 'users/:username',
		component: UserProfilePage,
		title: withTitlePrefix($localize`:@@routes.userProfile:User Profile`),
	},
	{
		path: 'reports',
		component: ReportsPage,
		canActivate: [authGuard],
		data: {roles: [Role.MODERATOR, Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.reports:Reports`),
	},
	{
		path: 'admin/users',
		component: AdminUsersPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.users:Users`),
	},
	{
		path: 'genres',
		component: GenresPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.genres:Genres`),
	},
	{
		path: 'tags',
		component: TagsPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.tags:Tags`),
	},
	{
		path: 'platforms',
		component: PlatformsPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.platforms:Platforms`),
	},
	{
		path: 'publishers',
		component: PublishersPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.publishers:Publishers`),
	},
	{
		path: 'developers',
		component: DevelopersPage,
		canActivate: [authGuard],
		data: {roles: [Role.ADMIN]},
		title: withTitlePrefix($localize`:@@routes.developers:Developers`),
	},
	{path: '', redirectTo: 'home', pathMatch: 'full'},
	{path: '**', redirectTo: 'home', pathMatch: 'full'},
];
