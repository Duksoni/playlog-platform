import {Component, effect, inject} from '@angular/core';
import {
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogRef,
	MatDialogTitle
} from '@angular/material/dialog';
import {FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {DialogService} from '../../../shared/services/dialog.service';
import {LoginService} from './login.service';
import {SessionService} from '../../../core/services/session.service';
import {Role} from '../auth.dto';
import {RegisterDialog} from '../register-dialog/register.dialog';
import {MatError, MatFormField, MatLabel, MatSuffix} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatIcon} from '@angular/material/icon';
import {MatButton, MatIconButton} from '@angular/material/button';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {MatTooltip} from '@angular/material/tooltip';

@Component({
	selector: 'app-login',
	imports: [
		MatDialogTitle,
		MatDialogContent,
		MatError,
		MatIcon,
		MatDialogActions,
		MatButton,
		MatDialogClose,
		MatProgressSpinner,
		MatFormField,
		MatLabel,
		MatInput,
		ReactiveFormsModule,
		MatIconButton,
		MatSuffix,
		MatTooltip
	],
	providers: [LoginService],
	templateUrl: './login.dialog.html',
	styleUrl: './login.dialog.css',
})
export class LoginDialog {
	private fb: FormBuilder = inject(FormBuilder);
	private dialogRef = inject(MatDialogRef<LoginDialog>);
	private dialogService = inject(DialogService);
	protected loginService = inject(LoginService);
	private sessionService = inject(SessionService);
	protected hidePassword = true;
	protected loginForm: FormGroup = this.fb.group({
		identifier: ['', [Validators.required, Validators.minLength(3)]],
		password: ['', [Validators.required, Validators.minLength(8)]]
	});

	constructor() {
		effect(() => {
			const user = this.sessionService.user();
			if (user.role == Role.GUEST) return;
			this.dialogRef.close();
		});
	}

	togglePasswordVisibility() {
		this.hidePassword = !this.hidePassword;
	}

	onLogin() {
		if (this.loginForm.valid) {
			this.loginService.login({
				identifier: this.loginForm.value.identifier,
				password: this.loginForm.value.password
			});
		}
	}

	openLoginDialog() {
		this.dialogRef.close();
		this.dialogService.openDialog(RegisterDialog, {
			disableClose: true,
			autoFocus: false,
		})
	}
}
