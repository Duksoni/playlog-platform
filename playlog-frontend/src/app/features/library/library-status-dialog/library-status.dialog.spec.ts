import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MAT_DIALOG_DATA, MatDialogRef} from '@angular/material/dialog';
import {LibraryService} from '../library.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {LibraryStatusDialog} from './library-status.dialog';

describe('LibraryStatusDialog', () => {
	let component: LibraryStatusDialog;
	let fixture: ComponentFixture<LibraryStatusDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [LibraryStatusDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MAT_DIALOG_DATA, useValue: {}},
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: LibraryService, useValue: {}},
        {provide: SnackbarService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(LibraryStatusDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
