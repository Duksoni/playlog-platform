import {ComponentFixture, TestBed} from '@angular/core/testing';
import {ReviewDialog} from './review.dialog';


describe('ReviewDialog', () => {
	let component: ReviewDialog;
	let fixture: ComponentFixture<ReviewDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [ReviewDialog]
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
