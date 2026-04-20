import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {
	FindUsersResponse,
	UpdatePasswordRequest,
	UpdateProfileRequest,
	UpdateUserStatusRequest,
	UserDetails,
	UserRoleChangeResponse,
} from './user.dto';
import {Role} from '../auth/auth.dto';

@Injectable({
	providedIn: 'root',
})
export class UserService {
	private http = inject(HttpClient);
	private readonly base = `${environment.apiUrl}/users`;

	getUser(username: string) {
		return this.http.get<UserDetails>(`${this.base}/${username}`);
	}

	updateProfile(body: UpdateProfileRequest) {
		return this.http.put<void>(`${this.base}/me`, body, {observe: 'response'});
	}

	changePassword(body: UpdatePasswordRequest) {
		return this.http.put<void>(`${this.base}/me/change-password`, body);
	}

	deactivateAccount(version: number) {
		const body: UpdateUserStatusRequest = {version};
		return this.http.delete<void>(`${this.base}/me`, {body});
	}

	findUsers(partialUsername: string, role: Role) {
		const params = new HttpParams()
			.set('partial_username', partialUsername)
			.set('role', role);
		return this.http.get<FindUsersResponse>(`${this.base}`, {params});
	}

	promoteUser(id: string, version: number) {
		const body: UpdateUserStatusRequest = {version};
		return this.http.put<UserRoleChangeResponse>(`${this.base}/${id}/promote`, body);
	}

	demoteUser(id: string, version: number) {
		const body: UpdateUserStatusRequest = {version};
		return this.http.put<UserRoleChangeResponse>(`${this.base}/${id}/demote`, body);
	}

	blockUser(id: string, version: number) {
		const body: UpdateUserStatusRequest = {version};
		return this.http.put<void>(`${this.base}/${id}/block`, body);
	}
}
