import {ComponentFixture, TestBed} from '@angular/core/testing';
import {GameDetailPage} from './game-detail.page';


describe('GameDetailPage', () => {
	let component: GameDetailPage;
	let fixture: ComponentFixture<GameDetailPage>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [GameDetailPage]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameDetailPage);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
