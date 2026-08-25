import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MatDialogRef} from '@angular/material/dialog';
import {DialogService} from '../../../shared/services/dialog.service';
import {SessionService} from '../../../core/services/session.service';
import {LoginDialog} from './login.dialog';

describe('Login', () => {
	let component: LoginDialog;
	let fixture: ComponentFixture<LoginDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [LoginDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MatDialogRef, useValue: {close: vi.fn()}},
        {provide: DialogService, useValue: {}},
        {provide: SessionService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(LoginDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
