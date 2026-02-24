export enum Role {
	GUEST = "GUEST",
	USER = "USER",
	MODERATOR = "MODERATOR",
	ADMIN = "ADMIN",
}

export interface TokenResponse {
	accessToken: string;
}

export interface UserClaims {
	userId: string; // user UUID
	role: Role;
	exp?: Date;
}

export interface LoginRequest {
	identifier: string;
	password: string;
}

export interface RegisterRequest {
	username: string;
	email: string;
	password: string;
	firstName?: string;
	lastName?: string;
	birthdate?: string;
}
