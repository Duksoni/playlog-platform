import {ComponentFixture, TestBed} from '@angular/core/testing';
import {LibrarySectionComponent} from './library-section.component';


describe('LibrarySectionComponent', () => {
	let component: LibrarySectionComponent;
	let fixture: ComponentFixture<LibrarySectionComponent>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [LibrarySectionComponent]
		})
			.compileComponents();

		fixture = TestBed.createComponent(LibrarySectionComponent);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
