import {ComponentFixture, TestBed} from '@angular/core/testing';

import {LibraryStatusDialog} from './library-status.dialog';

describe('LibraryStatusDialog', () => {
	let component: LibraryStatusDialog;
	let fixture: ComponentFixture<LibraryStatusDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [LibraryStatusDialog]
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
