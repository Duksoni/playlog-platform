import {ChangeDetectionStrategy, Component, inject, input, OnInit, signal} from '@angular/core';
import {GameEntitySimple, GameEntityType} from '../game-entity.dto';
import {GameEntityService} from '../game-entity.service';
import {MatTableModule} from '@angular/material/table';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatPaginatorModule, PageEvent} from '@angular/material/paginator';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {debounceTime, distinctUntilChanged} from 'rxjs';
import {DialogService} from '../../../shared/services/dialog.service';
import {GameEntityDialog} from '../game-entity-dialog/game-entity.dialog';
import {GameEntityDialogData} from '../game-entity-dialog/game-entity-dialog-data';
import {SnackbarService} from '../../../shared/services/snackbar.service';
import {ApiError} from '../../../core/api-error';
import {MatTooltip} from '@angular/material/tooltip';

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
	templateUrl: './game-entities-list.component.html',
	styleUrl: './game-entities-list.component.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameEntitiesListComponent implements OnInit {
	entityType = input.required<GameEntityType>();
	entityLabel = input.required<string>();
	allowDelete = input<boolean>(false);

	private entityService = inject(GameEntityService);
	private dialogService = inject(DialogService);
	private snackbarService = inject(SnackbarService);

	protected searchControl = new FormControl('');
	protected displayedColumns: string[] = ['name', 'actions'];

	// Local state
	protected items = signal<GameEntitySimple[]>([]);
	protected loading = signal(false);

	// Pagination state
	protected pageIndex = signal(0);
	protected pageSize = signal(10);
	protected totalItems = signal(0);

	ngOnInit() {
		this.loadData();

		this.searchControl.valueChanges.pipe(
			debounceTime(300),
			distinctUntilChanged()
		).subscribe(query => {
			if (query && query.length >= 2) {
				this.loading.set(true);
				this.entityService.search(this.entityType(), query).subscribe({
					next: (data) => {
						this.items.set(data);
						this.totalItems.set(data.length);
						this.loading.set(false);
					},
					error: () => this.loading.set(false),
				});
			} else if (!query) {
				this.loadData();
			}
		});
	}

	loadData() {
		this.loading.set(true);
		this.entityService.loadAll(this.entityType(), this.pageIndex(), this.pageSize()).subscribe({
			next: (response) => {
				this.items.set(response.data);
				this.totalItems.set(response.totalItems);
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	private resetAndLoad() {
		this.searchControl.setValue('');
		this.pageIndex.set(0);
		this.loadData();
	}

	handlePageEvent(e: PageEvent) {
		this.pageIndex.set(e.pageIndex);
		this.pageSize.set(e.pageSize);
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
					const apiError = err as ApiError;
					const errorMessage = apiError.error?.join('\n') || deleteFailed;
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
