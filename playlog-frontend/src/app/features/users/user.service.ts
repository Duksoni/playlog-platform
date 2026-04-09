import {inject, Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {UpdatePasswordRequest, UpdateProfileRequest, UserDetails} from './user.dto';

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

	deactivateAccount() {
		return this.http.delete<void>(`${this.base}/me`);
	}
}
