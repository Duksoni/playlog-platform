export enum Rating {
	NOT_RECOMMENDED = 'NOT_RECOMMENDED',
	OKAY = 'OKAY',
	GOOD = 'GOOD',
	HIGHLY_RECOMMENDED = 'HIGHLY_RECOMMENDED',
}

export interface CreateUpdateReviewRequest {
	gameId: number;
	rating: Rating;
	text?: string | null;
}

export interface ReviewDetailedResponse {
	id: string;
	gameId: number;
	userId: string;
	username: string;
	rating: Rating;
	text?: string | null;
	createdAt: string;
	updatedAt: string;
	version: number;
}

export interface ReviewSimpleResponse {
	id: string;
	gameId: number;
	userId: string;
	username: string;
	rating: Rating;
	text?: string | null;
	updatedAt: string;
}

export interface GameReviewResponse {
	id: string;
	userId: string;
	username: string;
	rating: Rating;
	text?: string | null;
	updatedAt: string;
}

export interface GameRatingStatsResponse {
	highlyRecommendedCount: number;
	goodCount: number;
	okayCount: number;
	notRecommendedCount: number;
}

// Display helpers

export const RATING_LABELS: Record<Rating, string> = {
	[Rating.NOT_RECOMMENDED]: $localize`:@@ratingNotRecommended:Not Recommended`,
	[Rating.OKAY]: $localize`:@@ratingOkay:Okay`,
	[Rating.GOOD]: $localize`:@@ratingGood:Good`,
	[Rating.HIGHLY_RECOMMENDED]: $localize`:@@ratingHighlyRecommended:Highly Recommended`,
};

export const RATING_ICONS: Record<Rating, string> = {
	[Rating.NOT_RECOMMENDED]: 'thumb_down',
	[Rating.OKAY]: 'thumbs_up_down',
	[Rating.GOOD]: 'thumb_up',
	[Rating.HIGHLY_RECOMMENDED]: 'thumbs_up_double',
};

// Ordered from most positive to least for display
export const RATING_ORDER: Rating[] = [
	Rating.HIGHLY_RECOMMENDED,
	Rating.GOOD,
	Rating.OKAY,
	Rating.NOT_RECOMMENDED,
];
