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
}

export interface DetailedCommentResponse {
	id: string;
	targetType: CommentTargetType;
	targetId: string;
	userId: string;
	username: string;
	text: string;
	createdAt: string;
}
