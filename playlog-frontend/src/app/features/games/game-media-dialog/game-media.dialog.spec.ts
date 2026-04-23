import {ComponentFixture, TestBed} from '@angular/core/testing';
import {GameMediaDialog} from './game-media.dialog';


describe('GameMediaDialog', () => {
	let component: GameMediaDialog;
	let fixture: ComponentFixture<GameMediaDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [GameMediaDialog]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameMediaDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
