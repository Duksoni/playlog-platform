import {ComponentFixture, TestBed} from '@angular/core/testing';
import {GamesListPage} from './games-list.page';


describe('GamesListPage', () => {
	let component: GamesListPage;
	let fixture: ComponentFixture<GamesListPage>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [GamesListPage]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GamesListPage);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
