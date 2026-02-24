import {Injectable} from '@angular/core';
import {MatDialog, MatDialogConfig} from '@angular/material/dialog';
import {SimpleDialogData} from '../components/simple-dialog/simple-dialog-data';
import {SimpleDialog} from '../components/simple-dialog/simple-dialog';
import {ComponentType} from '@angular/cdk/portal';

@Injectable({
	providedIn: 'root',
})
export class DialogService {
	constructor(private _dialog: MatDialog) {
	}

	public openSimpleDialog(config: MatDialogConfig<SimpleDialogData>) {
		return this._dialog.open(SimpleDialog, config);
	}

	public openDialog<T, D>(component: ComponentType<T>, config: MatDialogConfig<D>) {
		return this._dialog.open(component, config);
	}
}
