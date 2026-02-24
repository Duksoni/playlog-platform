import {Component, inject} from '@angular/core';
import {
	MAT_DIALOG_DATA,
	MatDialogActions,
	MatDialogClose,
	MatDialogContent,
	MatDialogTitle
} from '@angular/material/dialog';
import {SimpleDialogData} from './simple-dialog-data';
import {MatButton} from '@angular/material/button';

@Component({
	selector: 'app-simple-dialog',
	imports: [
		MatButton,
		MatDialogTitle,
		MatDialogContent,
		MatDialogActions,
		MatDialogClose
	],
	templateUrl: './simple-dialog.html',
	styleUrl: './simple-dialog.css',
})
export class SimpleDialog {
	data: SimpleDialogData = inject(MAT_DIALOG_DATA);

	protected positiveAction?: () => void;

	protected positiveActionText?: string;

	protected neutralAction?: () => void;

	protected neutralActionText?: string;

	protected negativeAction?: () => void;

	protected negativeActionText?: string;

	setPositiveButton(text: string, action: () => void) {
		this.positiveAction = action;
		this.positiveActionText = text;
	}

	setNeutralButton(text: string = "Close", action: () => void = () => {
	}) {
		this.neutralAction = action;
		this.neutralActionText = text;
	}

	setNegativeButton(text: string, action: () => void = () => {
	}) {
		this.negativeAction = action;
		this.negativeActionText = text;
	}
}
