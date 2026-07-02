import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MAT_DIALOG_DATA, MatDialogRef} from '@angular/material/dialog';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {GameEntityDialog} from './game-entity.dialog';

describe('GameEntityDialog', () => {
	let component: GameEntityDialog;
	let fixture: ComponentFixture<GameEntityDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [GameEntityDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MAT_DIALOG_DATA, useValue: {}},
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: SnackbarService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameEntityDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
