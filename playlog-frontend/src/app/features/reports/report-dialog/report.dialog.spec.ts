import {ComponentFixture, TestBed} from '@angular/core/testing';
import {ReportDialog} from './report.dialog';

describe('ReportDialog', () => {
	let component: ReportDialog;
	let fixture: ComponentFixture<ReportDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [ReportDialog]
		})
			.compileComponents();

		fixture = TestBed.createComponent(ReportDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
