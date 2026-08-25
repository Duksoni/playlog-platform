import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {provideRouter} from '@angular/router';
import {provideLocationMocks} from '@angular/common/testing';
import {ActivatedRoute} from '@angular/router';
import {GameService} from '../game.service';
import {SessionService} from '../../../core/services/session.service';
import {DialogService} from '../../../shared/services/dialog.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {LibraryService} from '../../library/library.service';
import {GameDetailPage} from './game-detail.page';


describe('GameDetailPage', () => {
	let component: GameDetailPage;
	let fixture: ComponentFixture<GameDetailPage>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [GameDetailPage],
      providers: [
        provideRouter([]),
        provideLocationMocks(),
        provideHttpClientTesting(),
        {provide: ActivatedRoute, useValue: {snapshot: {paramMap: {get: () => '1', has: () => true}}}},
        {provide: GameService, useValue: {}},
        {provide: SessionService, useValue: {}},
        {provide: DialogService, useValue: {}},
        {provide: SnackbarService, useValue: {}},
        {provide: LibraryService, useValue: {}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(GameDetailPage);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
