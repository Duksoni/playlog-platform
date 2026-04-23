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
		firstName: ['', []],
		lastName: ['', []],
		birthdate: [null, []],
		username: ['', [Validators.required, Validators.minLength(3), Validators.maxLength(50)]],
		email: ['', [Validators.required, Validators.email]],
		password: ['', [Validators.required, Validators.pattern(/^(?=.*[A-Z])(?=.*[a-z])(?=.*[@$!%*?&])(?=.*[0-9]).{8,}$/)]]
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
			const formValue = this.registerForm.value;
			const birthdate = formValue.birthdate instanceof Date
				? `${formValue.birthdate.getFullYear()}-${String(formValue.birthdate.getMonth() + 1).padStart(2, '0')}-${String(formValue.birthdate.getDate()).padStart(2, '0')}`
				: formValue.birthdate || null;

			this.registerService.register({
				username: formValue.username,
				email: formValue.email,
				password: formValue.password,
				firstName: formValue.firstName || null,
				lastName: formValue.lastName || null,
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
