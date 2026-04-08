import {
	ChangeDetectionStrategy,
	Component,
	computed,
	ElementRef,
	inject,
	NgZone,
	OnDestroy,
	OnInit,
	QueryList,
	signal,
	ViewChild,
	ViewChildren,
} from '@angular/core';
import {FormBuilder, ReactiveFormsModule} from '@angular/forms';
import {ActivatedRoute, Router} from '@angular/router';
import {debounceTime, distinctUntilChanged} from 'rxjs';
import {MatCardModule} from '@angular/material/card';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatSelectModule} from '@angular/material/select';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatTooltipModule} from '@angular/material/tooltip';
import {MatDividerModule} from '@angular/material/divider';
import {DatePipe} from '@angular/common';
import {GameService} from '../game.service';
import {SessionService} from '../../../core/services/session.service';
import {Role} from '../../auth/auth.dto';
import {DialogService} from '../../../shared/services/dialog.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {GameDialog} from '../game-dialog/game.dialog';
import {GameFilterParams} from '../game.dto';
import {
	SearchableMultiSelectComponent
} from '../../../shared/components/searchable-multi-select/searchable-multi-select.component';
import {MatSlideToggle, MatSlideToggleChange} from '@angular/material/slide-toggle';


@Component({
	selector: 'app-games-list-page',
	imports: [
		ReactiveFormsModule,
		MatCardModule,
		MatButtonModule,
		MatIconModule,
		MatFormFieldModule,
		MatInputModule,
		MatSelectModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		MatDividerModule,
		SearchableMultiSelectComponent,
		DatePipe,
		MatSlideToggle,
	],
	templateUrl: './games-list.page.html',
	styleUrl: './games-list.page.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GamesListPage implements OnInit, OnDestroy {
	@ViewChildren(SearchableMultiSelectComponent) private multiSelects!: QueryList<SearchableMultiSelectComponent>;

	@ViewChild('scrollSentinel') set sentinel(element: ElementRef<HTMLElement> | undefined) {
		if (element && this.observer) {
			this.observer.disconnect();
			this.observer.observe(element.nativeElement);
		}
	}

	protected gameService = inject(GameService);
	protected sessionService = inject(SessionService);
	private dialogService = inject(DialogService);
	private snackbarService = inject(SnackbarService);
	private fb = inject(FormBuilder);
	private router = inject(Router);
	private route = inject(ActivatedRoute);
	private zone = inject(NgZone);

	protected readonly Role = Role;
	protected readonly pageSize = 10;

	private pageIndex = 1;
	private hasMore = true;
	private observer: IntersectionObserver | null = null;

	protected selectedGenres = signal<number[]>([]);
	protected selectedPlatforms = signal<number[]>([]);
	protected selectedTags = signal<number[]>([]);

	protected developerId = signal<number | null>(null);
	protected publisherId = signal<number | null>(null);
	protected entityName = signal<string | null>(null);
	protected showOnlyDrafts = signal(false);

	protected isDeveloperRoute = computed(() => this.developerId() !== null);
	protected isPublisherRoute = computed(() => this.publisherId() !== null);
	protected isStandaloneList = computed(() => !this.isDeveloperRoute() && !this.isPublisherRoute());

	protected pageTitle = computed(() => {
		if (this.showOnlyDrafts()) return $localize`:@@games.manageUnpublished:Unpublished Games`;

		const name = this.entityName();
		if (this.isDeveloperRoute()) {
			return name
				? $localize`:@@games.byDeveloperName:Games by ${name}:name:`
				: $localize`:@@games.byDeveloper:Games by Developer`;
		}
		if (this.isPublisherRoute()) {
			return name
				? $localize`:@@games.byPublisherName:Games by ${name}:name:`
				: $localize`:@@games.byPublisher:Games by Publisher`;
		}
		return $localize`:@@games.catalogue:Game Catalogue`;
	});

	protected filterForm = this.fb.nonNullable.group({
		name: '',
		sort: 'name',
		sortDirection: this.fb.nonNullable.control<'asc' | 'desc'>('asc'),
	});

	protected activeFilterCount = computed(() =>
		this.selectedGenres().length +
		this.selectedPlatforms().length +
		this.selectedTags().length +
		(this.showOnlyDrafts() ? 1 : 0)
	);

	protected isAnyFilterActive = computed(() =>
		this.activeFilterCount() > 0 || !!this.filterForm.controls.name.value
	);

	ngOnInit() {
		this.setupIntersectionObserver();

		this.route.paramMap.subscribe(params => {
			const id = params.get('id');
			const path = this.route.snapshot.url[0]?.path;

			// Try to get name from router state
			const state = window.history.state;
			this.entityName.set(state?.name || null);

			this.developerId.set(null);
			this.publisherId.set(null);

			if (id && path === 'developers') {
				this.developerId.set(Number(id));
			} else if (id && path === 'publishers') {
				this.publisherId.set(Number(id));
			}

			this.resetAndLoad();
		});

		this.filterForm.controls.name.valueChanges.pipe(
			debounceTime(400),
			distinctUntilChanged(),
		).subscribe(() => this.resetAndLoad());

		this.filterForm.controls.sort.valueChanges.subscribe(() => this.resetAndLoad());
		this.filterForm.controls.sortDirection.valueChanges.subscribe(() => this.resetAndLoad());
	}

	ngOnDestroy() {
		this.observer?.disconnect();
	}

	private setupIntersectionObserver() {
		this.zone.runOutsideAngular(() => {
			this.observer = new IntersectionObserver(
				(entries) => {
					if (entries[0].isIntersecting && !this.gameService.loading() && this.hasMore) {
						this.zone.run(() => this.loadNextPage());
					}
				},
				{rootMargin: '200px'},
			);
		});
	}

	private buildParams(page: number): GameFilterParams {
		if (this.showOnlyDrafts()) {
			return {onlyDrafts: true};
		}
		if (this.developerId()) {
			return {developerId: this.developerId()!, page};
		}
		if (this.publisherId()) {
			return {publisherId: this.publisherId()!, page};
		}

		const v = this.filterForm.getRawValue();
		return {
			name: v.name || undefined,
			genres: this.selectedGenres().length ? this.selectedGenres() : undefined,
			platforms: this.selectedPlatforms().length ? this.selectedPlatforms() : undefined,
			tags: this.selectedTags().length ? this.selectedTags() : undefined,
			page,
			sort: v.sort,
			sortDirection: v.sortDirection,
		};
	}

	private loadFirst() {
		this.pageIndex = 1;
		// Correct initialization of hasMore to prevent duplication race conditions
		// onlyDrafts and developerId routes are currently not paginated
		this.hasMore = !this.showOnlyDrafts() && !this.isDeveloperRoute();

		this.gameService.filterGames(this.buildParams(1));
		this.watchResultLength();
	}

	private loadNextPage() {
		if (!this.hasMore || this.gameService.loading()) return;
		this.pageIndex++;
		this.gameService.filterGamesAppend(this.buildParams(this.pageIndex));
		this.watchResultLength();
	}

	private watchResultLength() {
		const check = () => {
			if (this.gameService.loading()) {
				setTimeout(check, 50);
				return;
			}
			const total = this.gameService.games().length;
			const expectedMin = this.pageIndex * this.pageSize;

			// Non-paginated routes
			if (this.showOnlyDrafts() || this.isDeveloperRoute()) {
				this.hasMore = false;
				return;
			}

			// For publisher and general catalogue: if we have fewer than full pages, we reached the end
			if (total < expectedMin) {
				this.hasMore = false;
			}
		};
		setTimeout(check, 50);
	}

	private resetAndLoad() {
		this.hasMore = true;
		this.loadFirst();
	}

	protected onGenresChange(ids: number[]) {
		this.selectedGenres.set(ids);
		this.resetAndLoad();
	}

	protected onPlatformsChange(ids: number[]) {
		this.selectedPlatforms.set(ids);
		this.resetAndLoad();
	}

	protected onTagsChange(ids: number[]) {
		this.selectedTags.set(ids);
		this.resetAndLoad();
	}

	protected onDraftToggle(event: MatSlideToggleChange) {
		this.showOnlyDrafts.set(event.checked);
		if (event.checked) {
			this.filterForm.disable({emitEvent: false});
		} else {
			this.filterForm.enable({emitEvent: false});
		}
		this.resetAndLoad();
	}

	protected clearFilters() {
		this.showOnlyDrafts.set(false);
		this.filterForm.enable({emitEvent: false});
		this.filterForm.patchValue({name: '', sort: 'name', sortDirection: 'asc'});
		this.selectedGenres.set([]);
		this.selectedPlatforms.set([]);
		this.selectedTags.set([]);
		this.multiSelects.forEach(c => c.reset());
		this.resetAndLoad();
	}

	protected openCreateDialog() {
		this.dialogService.openDialog(GameDialog, {
			data: {existing: null},
			width: '1000px',
			maxWidth: '50vw',
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			if (result) {
				this.resetAndLoad();
				this.snackbarService.createSnackbar($localize`:@@games.created:Game created successfully.`);
			}
		});
	}

	protected navigateToGame(id: number) {
		this.router.navigate(['/games', id]);
	}
}
