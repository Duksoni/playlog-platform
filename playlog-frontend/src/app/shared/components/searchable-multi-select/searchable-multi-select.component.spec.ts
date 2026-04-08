import {ComponentFixture, TestBed} from '@angular/core/testing';
import {SearchableMultiSelectComponent} from './searchable-multi-select.component';


describe('SearchableMultiSelectComponent', () => {
	let component: SearchableMultiSelectComponent;
	let fixture: ComponentFixture<SearchableMultiSelectComponent>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [SearchableMultiSelectComponent]
		})
			.compileComponents();

		fixture = TestBed.createComponent(SearchableMultiSelectComponent);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
