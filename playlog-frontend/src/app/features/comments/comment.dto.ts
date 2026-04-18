export enum CommentTargetType {
	GAME = 'GAME',
	REVIEW = 'REVIEW',
}

export interface CreateCommentRequest {
	targetType: CommentTargetType;
	targetId: string;
	text: string;
}

export interface UpdateCommentRequest {
	text: string;
}

export interface SimpleCommentResponse {
	id: string;
	userId: string;
	username: string;
	text: string;
	createdAt: string;
	updatedAt: string;
}

export interface DetailedCommentResponse extends  SimpleCommentResponse {
	targetType: CommentTargetType;
	targetId: string;
}

export interface RecentCommentResponse {
	id: string;
	gameId: number;
	username: string;
	text: string;
	createdAt: string;
	updatedAt: string;
}
