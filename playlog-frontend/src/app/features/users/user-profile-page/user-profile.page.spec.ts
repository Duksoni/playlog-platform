import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {provideRouter} from '@angular/router';
import {provideLocationMocks} from '@angular/common/testing';
import {ActivatedRoute} from '@angular/router';
import {UserService} from '../user.service';
import {SessionService} from '../../../core/services/session.service';
import {UserProfilePage} from './user-profile.page';

describe('UserProfilePage', () => {
	let component: UserProfilePage;
	let fixture: ComponentFixture<UserProfilePage>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [UserProfilePage],
      providers: [
        provideRouter([]),
        provideLocationMocks(),
        provideHttpClientTesting(),
        {provide: ActivatedRoute, useValue: {snapshot: {paramMap: {get: () => 'testuser', has: () => true}}}},
        {provide: UserService, useValue: {}},
        {provide: SessionService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(UserProfilePage);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
