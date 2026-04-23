import {ComponentFixture, TestBed} from '@angular/core/testing';

import {RegisterDialog} from './register.dialog';

describe('Register', () => {
	let component: RegisterDialog;
	let fixture: ComponentFixture<RegisterDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [RegisterDialog]
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
