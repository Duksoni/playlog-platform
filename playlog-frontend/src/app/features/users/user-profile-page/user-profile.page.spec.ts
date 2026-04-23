import {ComponentFixture, TestBed} from '@angular/core/testing';

import {UserProfilePage} from './user-profile.page';

describe('UserProfilePage', () => {
	let component: UserProfilePage;
	let fixture: ComponentFixture<UserProfilePage>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [UserProfilePage]
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
