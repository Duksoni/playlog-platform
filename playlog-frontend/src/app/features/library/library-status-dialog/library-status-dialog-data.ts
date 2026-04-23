import {GameLibraryStatus} from '../library.dto';

export interface LibraryStatusDialogData {
	gameId: number;
	gameName: string;
	currentStatus: GameLibraryStatus | null;
}
