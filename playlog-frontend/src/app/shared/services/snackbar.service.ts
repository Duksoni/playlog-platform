import {Injectable} from '@angular/core';
import {MatSnackBar, MatSnackBarConfig} from '@angular/material/snack-bar';

@Injectable({
	providedIn: 'root',
})
export class SnackbarService {
	constructor(private _snackbar: MatSnackBar) {
	}

	private readonly defaultConfig: MatSnackBarConfig = {
		horizontalPosition: 'center',
		verticalPosition: 'top',
		duration: 3500,
	};

	public createSnackbar(
		message: string,
		actionText?: string,
		actionCallback?: () => void,
		durationMs?: number,
	) {
		const snackBarRef = this._snackbar.open(
			message,
			actionText,
			durationMs ? {...this.defaultConfig, duration: durationMs} : this.defaultConfig
		);

		if (actionText && actionCallback) {
			snackBarRef.onAction().subscribe(() => actionCallback());
		}
	}
}
