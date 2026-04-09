import {ComponentFixture, TestBed} from '@angular/core/testing';

import {MyProfilePage} from './my-profile.page';

describe('MyProfilePage', () => {
	let component: MyProfilePage;
	let fixture: ComponentFixture<MyProfilePage>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [MyProfilePage]
		})
			.compileComponents();

		fixture = TestBed.createComponent(MyProfilePage);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
