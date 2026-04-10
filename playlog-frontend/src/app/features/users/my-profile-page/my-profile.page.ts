import {Component, inject, signal} from '@angular/core';
import {DatePipe} from '@angular/common';
import {MatButton, MatIconButton} from '@angular/material/button';
import {MatDatepicker, MatDatepickerInput, MatDatepickerToggle} from '@angular/material/datepicker';
import {MatDivider} from '@angular/material/list';
import {MatError, MatFormField, MatInput, MatLabel, MatSuffix} from '@angular/material/input';
import {MatIcon} from '@angular/material/icon';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {Router} from '@angular/router';
import {UserService} from '../user.service';
import {SessionService} from '../../../core/services/session.service';
import {DialogService} from '../../../shared/services/dialog.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {UserDetails} from '../user.dto';
import {ApiError} from '../../../core/api-error';
import {Role} from '../../auth/auth.dto';
import {MatTooltip} from '@angular/material/tooltip';
import {provideNativeDateAdapter} from '@angular/material/core';

@Component({
	selector: 'app-my-profile-page',
	imports: [
		DatePipe,
		MatButton,
		MatDatepicker,
		MatDatepickerInput,
		MatDatepickerToggle,
		MatDivider,
		MatError,
		MatFormField,
		MatIcon,
		MatIconButton,
		MatInput,
		MatLabel,
		MatProgressSpinner,
		MatSuffix,
		ReactiveFormsModule,
		MatTooltip
	],
	providers: [
		provideNativeDateAdapter()
	],
	templateUrl: './my-profile.page.html',
	styleUrl: './my-profile.page.css',
})
export class MyProfilePage {
	private router = inject(Router);
	private fb = inject(FormBuilder);
	private userService = inject(UserService);
	protected sessionService = inject(SessionService);
	private dialogService = inject(DialogService);
	private snackbarService = inject(SnackbarService);

	protected readonly Role = Role;

	protected user = signal<UserDetails | null>(null);
	protected loading = signal(true);

	protected profileSubmitting = signal(false);
	protected profileError = signal<ApiError | null>(null);

	protected passwordSubmitting = signal(false);
	protected passwordError = signal<ApiError | null>(null);
	protected hideOldPassword = true;
	protected hideNewPassword = true;

	protected profileForm!: FormGroup;
	protected passwordForm!: FormGroup;

	protected readonly maxDate = (() => {
		const date = new Date();
		date.setFullYear(date.getFullYear() - 16);
		return date;
	})();
	protected readonly minDate = (() => {
		const date = new Date();
		date.setFullYear(date.getFullYear() - 90);
		return date;
	})();

	ngOnInit() {
		const username = this.sessionService.user().username;
		if (!username) {
			this.router.navigate(['/home']);
			return;
		}

		this.userService.getUser(username).subscribe({
			next: (user) => {
				this.user.set(user);
				this.loading.set(false);
				this.buildProfileForm(user);
			},
			error: () => {
				this.loading.set(false);
				this.router.navigate(['/home']);
			},
		});

		this.passwordForm = this.fb.group({
			oldPassword: ['', [Validators.required, Validators.minLength(8)]],
			newPassword: ['', [Validators.required, Validators.pattern(/^(?=.*[A-Z])(?=.*[a-z])(?=.*[@$!%*?&])(?=.*[0-9]).{8,}$/)]]
		});
	}

	private buildProfileForm(user: UserDetails) {
		this.profileForm = this.fb.group({
			firstName: [
				user.firstName ?? '',
				this.user()!.firstName ? [Validators.required] : []
			],
			lastName: [
				user.lastName ?? '',
				this.user()!.lastName ? [Validators.required] : []
			],
			birthdate: [
				user.birthdate ? new Date(user.birthdate) : null,
				this.user()!.birthdate ? [Validators.required] : []
			],
		});
	}

	protected onProfileSubmit() {
		if (this.profileForm.invalid || this.profileSubmitting()) return;
		this.profileSubmitting.set(true);
		this.profileError.set(null);

		// Clear any previous form-level errors
		Object.keys(this.profileForm.controls).forEach(key => {
			this.profileForm.get(key)?.setErrors(null);
		});

		const formValue = this.profileForm.value;
		const birthdate = formValue.birthdate instanceof Date
			? `${formValue.birthdate.getFullYear()}-${String(formValue.birthdate.getMonth() + 1).padStart(2, '0')}-${String(formValue.birthdate.getDate()).padStart(2, '0')}`
			: formValue.birthdate || null;

		this.userService.updateProfile({
			firstName: formValue.firstName || null,
			lastName: formValue.lastName || null,
			birthdate,
		}).subscribe({
			next: (response) => {
				this.profileSubmitting.set(false);
				if (response.status === 204) {
					this.snackbarService.createSnackbar($localize`:@@profile.noChanges:No changes were made.`);
				} else {
					this.snackbarService.createSnackbar($localize`:@@profile.updated:Profile updated successfully.`);
				}
			},
			error: (err) => {
				this.profileSubmitting.set(false);
				const apiError = err as ApiError;
				this.profileError.set(apiError);

				apiError.error.forEach(errorMessage => {
					this.mapErrorToFormControl(errorMessage);
				});
			},
		});
	}

	private mapErrorToFormControl(errorMessage: string) {
		const fieldMap: Record<string, string> = {
			'firstName': 'firstName',
			'lastName': 'lastName',
			'birthdate': 'birthdate',
		};

		// Check which field the error is about
		for (const [key, controlName] of Object.entries(fieldMap)) {
			if (errorMessage.toLowerCase().includes(key.toLowerCase())) {
				const control = this.profileForm.get(controlName);
				if (control) {
					// Set a custom error with the backend message
					control.setErrors({backend: errorMessage});
				}
				break;
			}
		}
	}

	protected onPasswordSubmit() {
		if (this.passwordForm.invalid || this.passwordSubmitting()) return;
		this.passwordSubmitting.set(true);
		this.passwordError.set(null);

		this.userService.changePassword({
			oldPassword: this.passwordForm.value.oldPassword,
			newPassword: this.passwordForm.value.newPassword,
		}).subscribe({
			next: () => {
				this.passwordSubmitting.set(false);
				this.passwordForm.reset();
				this.snackbarService.createSnackbar($localize`:@@profile.passwordChanged:Password changed successfully.`);
			},
			error: (err) => {
				this.passwordSubmitting.set(false);
				this.passwordError.set(err as ApiError);
			},
		});
	}

	protected confirmDeactivate() {
		const dialogRef = this.dialogService.openSimpleDialog({
			width: '440px',
			data: {
				title: $localize`:@@profile.deactivateTitle:Deactivate Account`,
				content: $localize`:@@profile.deactivateContent:Are you sure you want to deactivate your account? You will be logged out and your profile will no longer be accessible.`,
			},
		});

		dialogRef.componentInstance.setPositiveButton(
			$localize`:@@profile.deactivateConfirm:Deactivate`,
			() => {
				this.userService.deactivateAccount().subscribe({
					next: () => {
						dialogRef.close();
						this.sessionService.handleLogout();
						this.router.navigate(['/home']);
						this.snackbarService.createSnackbar($localize`:@@profile.deactivated:Your account has been deactivated.`);
					},
					error: () => {
						dialogRef.close();
						this.snackbarService.createSnackbar($localize`:@@profile.deactivateFailed:Failed to deactivate account. Please try again.`);
					},
				});
			},
		);
		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`);
	}
}
