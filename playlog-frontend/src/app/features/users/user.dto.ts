export interface UserDetails {
	id: string;
	username: string;
	role: string;
	firstName?: string | null;
	lastName?: string | null;
	birthdate?: string | null;
	createdAt: string;
	version: number;
}

export interface UpdateProfileRequest {
	firstName?: string | null;
	lastName?: string | null;
	birthdate?: string | null;
	version: number;
}

export interface UpdatePasswordRequest {
	oldPassword: string;
	newPassword: string;
	version: number;
}

export interface UpdateUserStatusRequest {
	version: number;
}

export interface SimpleUser {
	id: string;
	username: string;
	version: number;
}

export interface FindUsersResponse {
	users: SimpleUser[];
}

export interface UserRoleChangeResponse {
	oldRole: string;
	newRole: string;
}
