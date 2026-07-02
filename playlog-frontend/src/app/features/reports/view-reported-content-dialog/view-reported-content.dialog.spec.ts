import {provideHttpClientTesting} from '@angular/common/http/testing';
import {ComponentFixture, TestBed} from '@angular/core/testing';
import {MAT_DIALOG_DATA} from '@angular/material/dialog';
import {of} from 'rxjs';
import {CommentService} from '../../comments/comment.service';
import {ReviewService} from '../../reviews/review.service';
import {ViewReportedContentDialog} from './view-reported-content.dialog';

describe('ViewReportedContentDialog', () => {
	let component: ViewReportedContentDialog;
	let fixture: ComponentFixture<ViewReportedContentDialog>;

	beforeEach(async () => {
		await TestBed.configureTestingModule({
      imports: [ViewReportedContentDialog],
      providers: [
        provideHttpClientTesting(),
        {provide: MAT_DIALOG_DATA, useValue: {targetType: 'REVIEW', targetId: '1'}},
        {provide: CommentService, useValue: {getComment: () => of({text: '', username: '', createdAt: null, updatedAt: null})}},
        {provide: ReviewService, useValue: {getReview: () => of({text: '', username: '', createdAt: null, updatedAt: null, rating: null})}},
      ]
		})
			.compileComponents();

		fixture = TestBed.createComponent(ViewReportedContentDialog);
		component = fixture.componentInstance;
		await fixture.whenStable();
	});

	it('should create', () => {
		expect(component).toBeTruthy();
	});
});
