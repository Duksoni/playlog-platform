import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {provideRouter} from '@angular/router';
import {provideLocationMocks} from '@angular/common/testing';
import {ActivatedRoute} from '@angular/router';
import {GameService} from '../game.service';
import {SessionService} from '../../../core/services/session.service';
import {DialogService} from '../../../shared/services/dialog.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {GamesListPage} from './games-list.page';


describe('GamesListPage', () => {
	let component: GamesListPage;
	let fixture: ComponentFixture<GamesListPage>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [GamesListPage],
      providers: [
        provideRouter([]),
        provideLocationMocks(),
        provideHttpClientTesting(),
        {provide: ActivatedRoute, useValue: {snapshot: {paramMap: {get: () => '1', has: () => true}}}},
        {provide: GameService, useValue: {}},
        {provide: SessionService, useValue: {}},
        {provide: DialogService, useValue: {}},
        {provide: SnackbarService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GamesListPage);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
