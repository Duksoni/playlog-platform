import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {provideRouter} from '@angular/router';
import {provideLocationMocks} from '@angular/common/testing';
import {SessionService} from '../../core/services/session.service';
import {DialogService} from '../../shared/services/dialog.service';
import {Navbar} from './navbar';

describe('Navbar', () => {
	let component: Navbar;
	let fixture: ComponentFixture<Navbar>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [Navbar],
      providers: [
        provideRouter([]),
        provideLocationMocks(),
        provideHttpClientTesting(),
        {provide: SessionService, useValue: {}},
        {provide: DialogService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(Navbar);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
