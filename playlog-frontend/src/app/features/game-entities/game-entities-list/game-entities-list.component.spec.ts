import {ComponentFixture, TestBed} from '@angular/core/testing';
import {GameEntitiesListComponent} from './game-entities-list.component';


describe('GameEntitiesListComponent', () => {
	let component: GameEntitiesListComponent;
	let fixture: ComponentFixture<GameEntitiesListComponent>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [GameEntitiesListComponent]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameEntitiesListComponent);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
