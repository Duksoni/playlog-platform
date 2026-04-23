import {ComponentFixture, TestBed} from '@angular/core/testing';
import {ViewReportedContentDialog} from './view-reported-content.dialog';

describe('ViewReportedContentDialog', () => {
	let component: ViewReportedContentDialog;
	let fixture: ComponentFixture<ViewReportedContentDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [ViewReportedContentDialog]
		})
			.compileComponents();

		fixture = TestBed.createComponent(ViewReportedContentDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
