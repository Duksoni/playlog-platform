import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MAT_DIALOG_DATA, MatDialogRef} from '@angular/material/dialog';
import {GameService} from '../game.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {GameMediaDialog} from './game-media.dialog';


describe('GameMediaDialog', () => {
	let component: GameMediaDialog;
	let fixture: ComponentFixture<GameMediaDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [GameMediaDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MAT_DIALOG_DATA, useValue: {}},
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: GameService, useValue: {}},
        {provide: SnackbarService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameMediaDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
