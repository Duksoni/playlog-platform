import {RecentReviewResponse} from '../reviews/review.dto';
import {RecentCommentResponse} from '../comments/comment.dto';

export interface RecentlyReviewedGame extends RecentReviewResponse {
	gameName: string;
}

export interface RecentlyCommentedGame extends RecentCommentResponse {
	gameName: string;
}
