import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GameEntityDialog } from './game-entity.dialog';

describe('GameEntityDialog', () => {
	let component: GameEntityDialog;
	let fixture: ComponentFixture<GameEntityDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [GameEntityDialog]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameEntityDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
