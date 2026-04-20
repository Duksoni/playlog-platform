import {ChangeDetectionStrategy, Component, inject, OnInit, signal} from '@angular/core';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {debounceTime, distinctUntilChanged, filter, startWith} from 'rxjs';
import {toSignal} from '@angular/core/rxjs-interop';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatSelectModule} from '@angular/material/select';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatTooltipModule} from '@angular/material/tooltip';
import {MatChipsModule} from '@angular/material/chips';
import {MatDividerModule} from '@angular/material/divider';
import {MatTableModule} from '@angular/material/table';
import {UserService} from '../user.service';
import {SimpleUser, UserRoleChangeResponse} from '../user.dto';
import {Role} from '../../auth/auth.dto';
import {DialogService} from '../../../shared/services/dialog.service';
import {SnackbarService} from '../../../shared/services/snackbar.service';

@Component({
	selector: 'app-admin-users-page',
	imports: [
		ReactiveFormsModule,
		MatButtonModule,
		MatIconModule,
		MatFormFieldModule,
		MatInputModule,
		MatSelectModule,
		MatProgressSpinnerModule,
		MatTooltipModule,
		MatChipsModule,
		MatDividerModule,
		MatTableModule,
	],
	templateUrl: './admin-users.page.html',
	styleUrl: './admin-users.page.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AdminUsersPage implements OnInit {
	private userService = inject(UserService);
	private dialogService = inject(DialogService);
	private snackbarService = inject(SnackbarService);

	protected readonly Role = Role;
	protected readonly searchableRoles = [Role.USER, Role.MODERATOR, Role.ADMIN];
	protected readonly displayedColumns = ['username', 'actions'];

	protected searchControl = new FormControl('');
	protected roleControl = new FormControl<Role>(Role.USER);

	protected searchQuery = toSignal(
		this.searchControl.valueChanges.pipe(startWith(this.searchControl.value ?? '')),
		{initialValue: this.searchControl.value ?? ''}
	);

	protected users = signal<SimpleUser[]>([]);
	protected loading = signal(false);
	protected actioningId = signal<string | null>(null);
	// Cache role per user id so we can show correct promote/demote buttons without re-fetching
	protected userRoles = signal<Record<string, string>>({});

	ngOnInit() {
		// Initial load with defaults
		this.search();

		this.searchControl.valueChanges.pipe(
			debounceTime(350),
			distinctUntilChanged(),
			filter(q => !q || q.trim().length !== 1)
		).subscribe(() => this.search());

		this.roleControl.valueChanges.subscribe(() => this.search());
	}

	private search() {
		const query = this.searchControl.value?.trim() ?? '';
		if (!query) {
			this.users.set([]);
			return;
		}
		if (query.length === 1) return;

		const role = this.roleControl.value ?? Role.USER;
		this.loading.set(true);

		this.userService.findUsers(query, role).subscribe({
			next: (response) => {
				this.users.set(response.users);
				// Seed the role cache with the selected filter role for all results
				const roleMap: Record<string, string> = {...this.userRoles()};
				response.users.forEach(u => {
					if (!roleMap[u.id]) roleMap[u.id] = role;
				});
				this.userRoles.set(roleMap);
				this.loading.set(false);
			},
			error: () => this.loading.set(false),
		});
	}

	protected promote(user: SimpleUser) {
		const currentRole = this.userRoles()[user.id] ?? this.roleControl.value;
		const nextRole = this.nextHigherRole(currentRole as Role);

		const dialogRef = this.dialogService.openSimpleDialog({
			width: '420px',
			autoFocus: false,
			disableClose: true,
			data: {
				title: $localize`:@@adminUsers.promoteTitle:Promote User`,
				content: $localize`:@@adminUsers.promoteContent:Promote ${user.username} from ${this.roleLabel(currentRole as Role)} to ${this.roleLabel(nextRole!)}?`,
			},
		});

		dialogRef.componentInstance.setPositiveButton(
			$localize`:@@adminUsers.confirmPromote:Promote`,
			() => {
				this.actioningId.set(user.id);
				this.userService.promoteUser(user.id, user.version).subscribe({
					next: (res: UserRoleChangeResponse) => {
						this.updateCachedRole(user.id, res.newRole);
						this.actioningId.set(null);
						this.snackbarService.createSnackbar(
							$localize`:@@adminUsers.promoted:${user.username} promoted to ${this.roleLabel(res.newRole as Role)}.`
						);
						dialogRef.close();
						this.search();
					},
					error: (err) => {
						this.actioningId.set(null);
						dialogRef.close();
						this.handleActionError(err);
					},
				});
			},
		);
		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`);
	}

	protected demote(user: SimpleUser) {
		const currentRole = this.userRoles()[user.id] ?? this.roleControl.value;
		const prevRole = this.nextLowerRole(currentRole as Role);

		const dialogRef = this.dialogService.openSimpleDialog({
			width: '420px',
			autoFocus: false,
			disableClose: true,
			data: {
				title: $localize`:@@adminUsers.demoteTitle:Demote User`,
				content: $localize`:@@adminUsers.demoteContent:Demote ${user.username} from ${this.roleLabel(currentRole as Role)} to ${this.roleLabel(prevRole!)}?`,
			},
		});

		dialogRef.componentInstance.setPositiveButton(
			$localize`:@@adminUsers.confirmDemote:Demote`,
			() => {
				this.actioningId.set(user.id);
				this.userService.demoteUser(user.id, user.version).subscribe({
					next: (res: UserRoleChangeResponse) => {
						this.updateCachedRole(user.id, res.newRole);
						this.actioningId.set(null);
						this.snackbarService.createSnackbar(
							$localize`:@@adminUsers.demoted:${user.username} demoted to ${this.roleLabel(res.newRole as Role)}.`
						);
						dialogRef.close();
						this.search();
					},
					error: (err) => {
						this.actioningId.set(null);
						dialogRef.close();
						this.handleActionError(err);
					},
				});
			},
		);
		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`);
	}

	protected block(user: SimpleUser) {
		const dialogRef = this.dialogService.openSimpleDialog({
			width: '420px',
			autoFocus: false,
			disableClose: true,
			data: {
				title: $localize`:@@adminUsers.blockTitle:Block User`,
				content: $localize`:@@adminUsers.blockContent:Block ${user.username}? They will no longer be able to log in.`,
			},
		});

		dialogRef.componentInstance.setPositiveButton(
			$localize`:@@adminUsers.confirmBlock:Block`,
			() => {
				this.actioningId.set(user.id);
				this.userService.blockUser(user.id, user.version).subscribe({
					next: () => {
						this.actioningId.set(null);
						this.snackbarService.createSnackbar(
							$localize`:@@adminUsers.blocked:${user.username} has been blocked.`
						);
						dialogRef.close();
						this.search();
					},
					error: (err) => {
						this.actioningId.set(null);
						dialogRef.close();
						this.handleActionError(err);
					},
				});
			},
		);
		dialogRef.componentInstance.setNegativeButton($localize`:@@common.cancel:Cancel`);
	}

	private updateCachedRole(userId: string, newRole: string) {
		this.userRoles.update(map => ({...map, [userId]: newRole}));
	}

	private handleActionError(err: { status: number }) {
		if (err.status === 400) {
			this.snackbarService.createSnackbar(
				$localize`:@@adminUsers.actionInvalid:This action is not allowed (e.g. self-promotion or user is blocked).`
			);
		} else {
			this.snackbarService.createSnackbar(
				$localize`:@@adminUsers.actionFailed:Action failed. Please try again.`
			);
		}
	}

	protected roleLabel(role: Role | string): string {
		switch (role) {
			case Role.ADMIN:
				return $localize`:@@role.admin:Admin`;
			case Role.MODERATOR:
				return $localize`:@@role.moderator:Moderator`;
			default:
				return $localize`:@@role.user:User`;
		}
	}

	protected roleBadgeClass(role: Role | string): string {
		switch (role) {
			case Role.ADMIN:
				return 'role-admin';
			case Role.MODERATOR:
				return 'role-moderator';
			default:
				return 'role-user';
		}
	}

	// Can this user be promoted further?
	protected canPromote(userId: string): boolean {
		const role = this.userRoles()[userId] ?? this.roleControl.value;
		return role !== Role.ADMIN;
	}

	// Can this user be demoted further?
	protected canDemote(userId: string): boolean {
		const role = this.userRoles()[userId] ?? this.roleControl.value;
		return role !== Role.USER;
	}

	private nextHigherRole(role: Role): Role | null {
		if (role === Role.USER) return Role.MODERATOR;
		if (role === Role.MODERATOR) return Role.ADMIN;
		return null;
	}

	private nextLowerRole(role: Role): Role | null {
		if (role === Role.ADMIN) return Role.MODERATOR;
		if (role === Role.MODERATOR) return Role.USER;
		return null;
	}
}
