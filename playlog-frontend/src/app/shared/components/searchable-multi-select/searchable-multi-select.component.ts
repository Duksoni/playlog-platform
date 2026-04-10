import {
	ChangeDetectionStrategy,
	Component,
	computed,
	effect,
	inject,
	input,
	OnInit,
	output,
	signal,
} from '@angular/core';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatAutocompleteModule, MatAutocompleteSelectedEvent} from '@angular/material/autocomplete';
import {MatChipsModule} from '@angular/material/chips';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {debounceTime, distinctUntilChanged} from 'rxjs';
import {GameEntityService} from '../../../features/game-entities/game-entity.service';
import {GameEntitySimple, GameEntityType} from '../../../features/game-entities/game-entity.dto';

@Component({
	selector: 'app-searchable-multi-select',
	imports: [
		ReactiveFormsModule,
		MatFormFieldModule,
		MatInputModule,
		MatAutocompleteModule,
		MatChipsModule,
		MatIconModule,
		MatProgressSpinnerModule,
	],
	templateUrl: './searchable-multi-select.component.html',
	styleUrl: './searchable-multi-select.component.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SearchableMultiSelectComponent implements OnInit {
	entityType = input.required<GameEntityType>();
	label = input.required<string>();
	placeholder = computed(() => this.label() + '...');
	initialValue = input<GameEntitySimple[]>([]);
	disabled = input<boolean>(false);

	selectedIdsChange = output<number[]>();

	private entityService = inject(GameEntityService);

	protected searchControl = new FormControl('');
	protected options = signal<GameEntitySimple[]>([]);
	protected optionsLoading = signal(false);

	private selectedMap = new Map<number, GameEntitySimple>();
	protected selectedItems = signal<GameEntitySimple[]>([]);

	protected filteredOptions = computed(() => this.options());

	constructor() {
		effect(() => {
			if (this.disabled()) {
				this.searchControl.disable({ emitEvent: false });
			} else {
				this.searchControl.enable({ emitEvent: false });
			}
		});
	}

	ngOnInit() {
		const initial = this.initialValue();
		if (initial.length > 0) {
			initial.forEach(item => {
				this.selectedMap.set(item.id, item);
			});
			this.selectedItems.set([...this.selectedMap.values()]);
		}

		this.fetchOptions('');

		this.searchControl.valueChanges.pipe(
			debounceTime(300),
			distinctUntilChanged(),
		).subscribe(value => {
			const query = typeof value === 'string' ? value : '';
			this.fetchOptions(query);
		});
	}

	reset() {
		this.selectedMap.clear();
		this.selectedItems.set([]);
		this.emitChange();
		this.searchControl.setValue('', {emitEvent: false});
		this.fetchOptions('');
	}

	private fetchOptions(query: string | null) {
		this.optionsLoading.set(true);
		const searchBox = (query || '').trim();
		const obs = searchBox.length >= 1
			? this.entityService.searchForFilter(this.entityType(), searchBox)
			: this.entityService.getAllForFilter(this.entityType());

		obs.subscribe({
			next: (items) => {
				this.options.set(items);
				this.optionsLoading.set(false);
			},
			error: () => this.optionsLoading.set(false),
		});
	}

	protected onOptionSelected(event: MatAutocompleteSelectedEvent) {
		const item = event.option.value as GameEntitySimple;
		if (this.selectedMap.has(item.id)) {
			this.selectedMap.delete(item.id);
		} else {
			this.selectedMap.set(item.id, item);
		}
		this.selectedItems.set([...this.selectedMap.values()]);
		this.emitChange();
		this.searchControl.setValue('', {emitEvent: false});
		this.fetchOptions('');
	}

	protected removeItem(id: number) {
		this.selectedMap.delete(id);
		this.selectedItems.set([...this.selectedMap.values()]);
		this.emitChange();
	}

	protected isSelected(id: number): boolean {
		return this.selectedMap.has(id);
	}

	private emitChange() {
		this.selectedIdsChange.emit([...this.selectedMap.keys()]);
	}

	protected displayFn = (): string => '';
}
