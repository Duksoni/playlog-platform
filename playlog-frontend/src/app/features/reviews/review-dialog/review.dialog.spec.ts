import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MAT_DIALOG_DATA, MatDialogRef} from '@angular/material/dialog';
import {ReviewService} from '../review.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ReviewDialog} from './review.dialog';


describe('ReviewDialog', () => {
	let component: ReviewDialog;
	let fixture: ComponentFixture<ReviewDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [ReviewDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MAT_DIALOG_DATA, useValue: {}},
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: ReviewService, useValue: {}},
        {provide: SnackbarService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(ReviewDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
