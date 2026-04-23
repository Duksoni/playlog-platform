export enum Role {
	GUEST = "GUEST",
	USER = "USER",
	MODERATOR = "MODERATOR",
	ADMIN = "ADMIN",
}

export interface TokenResponse {
	accessToken: string;
}

export interface TokenPayload {
	sub: string, // Subject (user UUID)
	exp: number,  // Expiration time (as UTC timestamp)
	iat: number,  // Issued at (as UTC timestamp)
	iss: string, // Issuer
	role: Role,
	username: string,
	email: string,
}

export interface UserClaims {
	userId: string; // user UUID
	username: string;
	email: string;
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
