import {TestBed} from '@angular/core/testing';
import {GameEntityService} from './game-entity.service';

describe('GameEntityService', () => {
	let service: GameEntityService;

	beforeEach(() => {
		TestBed.configureTestingModule({});
		service = TestBed.inject(GameEntityService);
	});

	it('should be created', () => {
		expect(service).toBeTruthy();
	});
});
