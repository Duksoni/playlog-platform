import {GameEntity, GameEntityType} from '../game-entity.dto';

export interface GameEntityDialogData {
	entityType: GameEntityType;
	entityLabel: string;     // e.g. "Genre", "Platform"
	existing?: GameEntity;   // present when editing, absent when creating
}
