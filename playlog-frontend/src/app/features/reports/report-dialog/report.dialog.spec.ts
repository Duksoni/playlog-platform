import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MAT_DIALOG_DATA, MatDialogRef} from '@angular/material/dialog';
import {ReportService} from '../report.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ReportDialog} from './report.dialog';

describe('ReportDialog', () => {
	let component: ReportDialog;
	let fixture: ComponentFixture<ReportDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [ReportDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MAT_DIALOG_DATA, useValue: {}},
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: ReportService, useValue: {}},
        {provide: SnackbarService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(ReportDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
