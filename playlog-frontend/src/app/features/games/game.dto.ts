import {GameEntitySimple} from '../game-entities/game-entity.dto';

export interface Game {
	id: number;
	name: string;
	description: string;
	released?: string | null;
	website?: string | null;
	draft: boolean;
	version: number;
}

export interface GameSimple {
	id: number;
	name: string;
	released?: string | null;
	draft: boolean;
}

export interface GameCard extends GameSimple {
	cover: string | null;
}

export interface GetGameCoversResponse {
	gameCovers: Record<number, string | null>;
}

export type GameDetails = Game & {
	developers: GameEntitySimple[];
	publishers: GameEntitySimple[];
	platforms: GameEntitySimple[];
	genres: GameEntitySimple[];
	tags: GameEntitySimple[];
};

export interface MediaFileResponse {
	url: string;
	mimeType: string;
	sizeBytes: number;
}

export interface GameMediaResponse {
	gameId: number;
	cover?: MediaFileResponse | null;
	screenshots: MediaFileResponse[];
	trailer?: MediaFileResponse | null;
	version: number;
}

export interface CreateGameRequest {
	name: string;
	description: string;
	released?: string | null;
	website?: string | null;
	developers: number[];
	publishers: number[];
	genres: number[];
	platforms: number[];
	tags: number[];
}

export interface UpdateGameRequest {
	name: string;
	description: string;
	released?: string | null;
	website?: string | null;
	version: number;
	developers?: number[] | null;
	publishers?: number[] | null;
	genres?: number[] | null;
	platforms?: number[] | null;
	tags?: number[] | null;
}

export interface PublishUnpublishGameRequest {
	version: number;
}

export interface GameFilterParams {
	name?: string;
	platforms?: number[];
	genres?: number[];
	tags?: number[];
	developerId?: number;
	publisherId?: number;
	onlyDrafts?: boolean;
	page?: number;
	sort?: string;
	sortDirection?: string;
}
