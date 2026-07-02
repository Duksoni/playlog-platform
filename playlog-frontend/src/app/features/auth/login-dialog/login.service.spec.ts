import {provideHttpClientTesting} from '@angular/common/http/testing';
import {TestBed} from '@angular/core/testing';
import {SessionService} from '../../../core/services/session.service';
import {LoginService} from './login.service';

describe('LoginService', () => {
	let service: LoginService;

	beforeEach(() => {
		TestBed.configureTestingModule({
			providers: [
				provideHttpClientTesting(),
				{provide: SessionService, useValue: {}},
			],
		});
		service = TestBed.inject(LoginService);
	});

	it('should be created', () => {
		expect(service).toBeTruthy();
	});
});
