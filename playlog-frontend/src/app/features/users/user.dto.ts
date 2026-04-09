export interface UserDetails {
	id: string;
	username: string;
	role: string;
	firstName?: string | null;
	lastName?: string | null;
	birthdate?: string | null;
	createdAt: string;
}

export interface UpdateProfileRequest {
	firstName?: string | null;
	lastName?: string | null;
	birthdate?: string | null;
}

export interface UpdatePasswordRequest {
	oldPassword: string;
	newPassword: string;
}
