export enum GameLibraryStatus {
	OWNED = 'OWNED',
	PLAYING = 'PLAYING',
	WISHLIST = 'WISHLIST',
	COMPLETED = 'COMPLETED',
	DROPPED = 'DROPPED',
}

export interface LibraryGame {
	gameId: number;
	status: GameLibraryStatus;
	addedAt: string;
	updatedAt: string;
}

export interface LibraryGameCard extends LibraryGame {
	cover: string | null;
	name: string;
	released: string | null;
}

export interface UserGame {
	userId: string;
	gameId: number;
	status: GameLibraryStatus;
	addedAt: string;
	updatedAt: string;
}

export interface AddUpdateGameRequest {
	gameId: number;
	status: GameLibraryStatus;
}

export type LibraryStats = Partial<Record<GameLibraryStatus, number>>;

// Labels for display
export const LIBRARY_STATUS_LABELS: Record<GameLibraryStatus, string> = {
	[GameLibraryStatus.OWNED]: $localize`:@@libraryOwned:Owned`,
	[GameLibraryStatus.PLAYING]: $localize`:@@libraryPlaying:Playing`,
	[GameLibraryStatus.WISHLIST]: $localize`:@@libraryWishlist:Wishlist`,
	[GameLibraryStatus.COMPLETED]: $localize`:@@libraryCompleted:Completed`,
	[GameLibraryStatus.DROPPED]: $localize`:@@libraryDropped:Dropped`,
};

export const LIBRARY_STATUS_ICONS: Record<GameLibraryStatus, string> = {
	[GameLibraryStatus.OWNED]: 'library_add_check',
	[GameLibraryStatus.PLAYING]: 'sports_esports',
	[GameLibraryStatus.WISHLIST]: 'bookmark',
	[GameLibraryStatus.COMPLETED]: 'check_circle',
	[GameLibraryStatus.DROPPED]: 'cancel',
};
