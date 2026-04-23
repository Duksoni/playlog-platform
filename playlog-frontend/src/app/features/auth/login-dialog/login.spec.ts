import {ComponentFixture, TestBed} from '@angular/core/testing';

import {LoginDialog} from './login.dialog';

describe('Login', () => {
	let component: LoginDialog;
	let fixture: ComponentFixture<LoginDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [LoginDialog]
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
