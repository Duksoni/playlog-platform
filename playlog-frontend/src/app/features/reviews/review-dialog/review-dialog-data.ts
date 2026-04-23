import {ReviewSimpleResponse} from '../review.dto';

export interface ReviewDialogData {
	gameId: number;
	gameName: string;
	existing: ReviewSimpleResponse | null;
}
