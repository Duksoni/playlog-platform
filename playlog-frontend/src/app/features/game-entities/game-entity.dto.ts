export interface GameEntity {
	id: number;
	name: string;
	version: number;
}

export interface GameEntitySimple {
	id: number;
	name: string;
}

export interface CreateGameEntityRequest {
	name: string;
}

export interface UpdateGameEntityRequest {
	name: string;
	version: number;
}

export type GameEntityType = 'genres' | 'tags' | 'platforms' | 'publishers' | 'developers';
