import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MatDialogRef} from '@angular/material/dialog';
import {DialogService} from '../../../shared/services/dialog.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {RegisterDialog} from './register.dialog';

describe('Register', () => {
	let component: RegisterDialog;
	let fixture: ComponentFixture<RegisterDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [RegisterDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: DialogService, useValue: {}},
        {provide: SnackbarService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(RegisterDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
