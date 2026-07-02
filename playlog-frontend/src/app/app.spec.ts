import {provideHttpClientTesting} from '@angular/common/http/testing';
import {TestBed} from '@angular/core/testing';
import {provideRouter} from '@angular/router';
import {provideLocationMocks} from '@angular/common/testing';
import {SessionService} from './core/services/session.service';
import {DialogService} from './shared/services/dialog.service';
import {App} from './app';

describe('App', () => {
	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [App],
      providers: [
        provideRouter([]),
        provideLocationMocks(),
        provideHttpClientTesting(),
        {provide: SessionService, useValue: {}},
        {provide: DialogService, useValue: {}},
      ],
		}).compileComponents();
	});

	it('should create the app', () => {
		const fixture = TestBed.createComponent(App);
		const app = fixture.componentInstance;
		expect(app).toBeTruthy();
	});


});
