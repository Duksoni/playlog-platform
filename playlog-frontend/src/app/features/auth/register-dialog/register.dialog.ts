import {Component, effect, inject} from '@angular/core';
import {
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle
} from '@angular/material/dialog';
import {FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatError, MatFormField, MatLabel, MatSuffix} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatDatepicker, MatDatepickerInput, MatDatepickerToggle} from '@angular/material/datepicker';
import {MatIcon} from '@angular/material/icon';
import {MatButton, MatIconButton} from '@angular/material/button';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {LoginDialog} from '../login-dialog/login.dialog';
import {DialogService} from '../../../shared/services/dialog.service';
import {RegisterService} from './register.service';
import {provideNativeDateAdapter} from '@angular/material/core';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {MatTooltip} from '@angular/material/tooltip';

@Component({
	selector: 'app-register',
	imports: [
		MatDialogTitle,
		MatDialogContent,
		ReactiveFormsModule,
		MatFormField,
		MatLabel,
		MatError,
		MatInput,
		MatDatepickerInput,
		MatDatepickerToggle,
		MatIcon,
		MatDatepicker,
		MatDialogActions,
		MatButton,
		MatDialogClose,
		MatProgressSpinner,
		MatIconButton,
		MatSuffix,
		MatTooltip,
	],
	providers: [RegisterService, provideNativeDateAdapter()],
	templateUrl: './register.dialog.html',
	styleUrl: './register.dialog.css',
})
export class RegisterDialog {
	private fb = inject(FormBuilder);
	private dialogService = inject(DialogService);
	private dialogRef = inject(MatDialogRef<RegisterDialog>);
	private snackbarService = inject(SnackbarService);
	protected registerService = inject(RegisterService);
	protected hidePassword = true;
	protected minDate = new Date();
	protected maxDate = new Date();

	registerForm: FormGroup = this.fb.group({
		firstName: ['', [Validators.required]],
		lastName: ['', [Validators.required]],
		birthdate: [null, [Validators.required]],
		username: ['', [Validators.required, Validators.minLength(3), Validators.maxLength(50)]],
		email: ['', [Validators.required, Validators.email]],
		password: ['', [Validators.required, Validators.pattern(/^(?=.*[A-Z])(?=.*[@$!%*?&])(?=.*[0-9]).{8,}$/)]]
	});

	startDate = new Date("2000-01-01");

	constructor() {
		this.maxDate.setFullYear(new Date().getFullYear() - 16);
		this.minDate.setFullYear(new Date().getFullYear() - 90);
		effect(() => {
			if (this.registerService.success()) {
				this.dialogRef.close();
				this.snackbarService.createSnackbar("Registration successful! You can now log in.");
			}
		});
	}

	protected togglePasswordVisibility() {
		this.hidePassword = !this.hidePassword;
	}

	protected onRegister() {
		if (this.registerForm.valid) {
			let {first_name, last_name, birthdate, username, email, password} = this.registerForm.value;
			if (birthdate instanceof Date) {
				const year = birthdate.getFullYear();
				const month = String(birthdate.getMonth() + 1).padStart(2, '0');
				const day = String(birthdate.getDate()).padStart(2, '0');
				birthdate = `${year}-${month}-${day}`;
			}

			this.registerService.register({
				username: username,
				email: email,
				password: password,
				firstName: first_name,
				lastName: last_name,
				birthdate: birthdate
			});
		}
	}

	protected openLoginDialog() {
		this.dialogRef.close();
		this.dialogService.openDialog(LoginDialog, {
			width: '450px',
			disableClose: true,
			autoFocus: false,
		})
	}
}
