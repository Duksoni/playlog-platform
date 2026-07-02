import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MAT_DIALOG_DATA, MatDialogRef} from '@angular/material/dialog';
import {GameService} from '../game.service';
import {GameDialog} from './game.dialog';


describe('GameDialog', () => {
	let component: GameDialog;
	let fixture: ComponentFixture<GameDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [GameDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MAT_DIALOG_DATA, useValue: {}},
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: GameService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
