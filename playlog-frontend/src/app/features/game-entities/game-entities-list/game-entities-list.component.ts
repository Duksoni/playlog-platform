import {ChangeDetectionStrategy, Component, effect, inject, input, OnInit, signal} from '@angular/core';
import {GameEntitySimple, GameEntityType} from '../game-entity.dto';
import {GameEntityService} from '../game-entity.service';
import {MatTableModule} from '@angular/material/table';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatPaginatorIntl, MatPaginatorModule, PageEvent} from '@angular/material/paginator';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {debounceTime, distinctUntilChanged} from 'rxjs';
import {DialogService} from '../../../shared/services/dialog.service';
import {GameEntityDialog} from '../game-entity-dialog/game-entity.dialog';
import {GameEntityDialogData} from '../game-entity-dialog/game-entity-dialog-data';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ApiError} from '../../../core/api-error';
import {MatTooltip} from '@angular/material/tooltip';
import {UnknownTotalCountPaginatorIntl} from '../../../shared/unknown-total-count.paginator';

@Component({
	selector: 'app-game-entities-list',
	standalone: true,
	imports: [
		MatTableModule,
		MatButtonModule,
		MatIconModule,
		MatProgressSpinnerModule,
		MatFormFieldModule,
		MatInputModule,
		MatPaginatorModule,
		ReactiveFormsModule,
		MatTooltip,
	],
	providers: [
		{provide: MatPaginatorIntl, useClass: UnknownTotalCountPaginatorIntl},
	],
	templateUrl: './game-entities-list.component.html',
	styleUrl: './game-entities-list.component.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameEntitiesListComponent implements OnInit {
	entityType = input.required<GameEntityType>();
	entityLabel = input.required<string>();
	allowDelete = input<boolean>(false);

	protected entityService = inject(GameEntityService);
	private dialogService = inject(DialogService);
	private snackbarService = inject(SnackbarService);

	protected searchControl = new FormControl('');
	protected displayedColumns: string[] = ['name', 'actions'];

	// Pagination state
	protected pageIndex = signal(0);
	protected readonly pageSize = 10;
	protected totalItems = signal(Number.MAX_SAFE_INTEGER);

	constructor() {
		effect(() => {
			const items = this.entityService.items();
			if (this.searchControl.value) return;

			const currentIdx = this.pageIndex();
			if (items.length === 0) {
				// We found the last page
				this.totalItems.set(currentIdx * this.pageSize);
			} else if (items.length < this.pageSize) {
				// We found the exact count
				this.totalItems.set(currentIdx * this.pageSize + items.length);
			} else {
				// If we don't know the total yet, keep it as 'many'
				if (this.totalItems() !== Number.MAX_SAFE_INTEGER) {
					this.totalItems.set(Number.MAX_SAFE_INTEGER);
				}
			}
		});
	}

	ngOnInit() {
		this.loadData();

		this.searchControl.valueChanges.pipe(
			debounceTime(300),
			distinctUntilChanged()
		).subscribe(query => {
			if (query && query.length >= 2) {
				this.entityService.search(this.entityType(), query);
			} else if (!query) {
				this.loadData();
			}
		});
	}

	loadData() {
		this.entityService.loadAll(this.entityType(), this.pageIndex());
	}

	private resetAndLoad() {
		this.searchControl.setValue('');
		this.pageIndex.set(0);
		this.loadData();
	}

	handlePageEvent(e: PageEvent) {
		this.pageIndex.set(e.pageIndex);
		this.loadData();
	}

	openCreateDialog() {
		const data: GameEntityDialogData = {
			entityType: this.entityType(),
			entityLabel: this.entityLabel(),
		};

		this.dialogService.openDialog(GameEntityDialog, {
			data,
			width: '400px',
			disableClose: true,
			autoFocus: false,
		}).afterClosed().subscribe(result => {
			if (result) {
				this.resetAndLoad();
			}
		});
	}

	openEditDialog(item: GameEntitySimple) {
		this.entityService.getById(this.entityType(), item.id).subscribe(fullEntity => {
			const data: GameEntityDialogData = {
				entityType: this.entityType(),
				entityLabel: this.entityLabel(),
				existing: fullEntity,
			};

			this.dialogService.openDialog(GameEntityDialog, {
				data,
				width: '400px',
				disableClose: true,
				autoFocus: false,
			}).afterClosed().subscribe(result => {
				if (result) {
					this.resetAndLoad();
				}
			});
		});
	}

	openDeleteDialog(item: GameEntitySimple) {
		const dialogRef = this.dialogService.openSimpleDialog({
			width: '400px',
			disableClose: true,
			autoFocus: false,
			data: {
				title: $localize`:@@deleteDialog.title:Delete ${this.entityLabel()}:entityLabel:`,
				content: $localize`:@@deleteDialog.content:Are you sure you want to delete ${this.entityLabel()}:entityLabel: "${item.name}"? This action cannot be undone.`,
			},
		});

		dialogRef.componentInstance.setPositiveButton($localize`:@@common.delete:Delete`, () => {
			this.entityService.delete(this.entityType(), item.id).subscribe({
				next: () => {
					const deleteSuccess = $localize`:@@deletedSuccessfully:deleted successfully.`;
					this.snackbarService.createSnackbar(`${this.entityLabel()} ${deleteSuccess}`);
					this.resetAndLoad();
					dialogRef.close();
				},
				error: (err) => {
					const deleteFailed = $localize`:@@deleteFailed:Failed to delete.`;
					const apiError = err.error as ApiError;
					const errorMessage = apiError.errors?.join('\n') || deleteFailed;
					this.snackbarService.createSnackbar(errorMessage);
					dialogRef.close();
				},
			});
		});

		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`, () => {
			dialogRef.close();
		});
	}
}
