import {Injectable} from '@angular/core';
import {MatPaginatorIntl} from '@angular/material/paginator';

@Injectable()
export class UnknownTotalCountPaginatorIntl extends MatPaginatorIntl {
	override itemsPerPageLabel = $localize`:@@itemsPerPageLabel:Items per page:`;
	override nextPageLabel = $localize`:@@nextPageLabel:Next page`;
	override previousPageLabel = $localize`:@@previousPageLabel:Previous page`;

	override getRangeLabel = (page: number, pageSize: number, length: number) => {
		const of = $localize`:@@of:of`;
		if (length === 0 || pageSize === 0) {
			return `0 ${of} 0`;
		}
		length = Math.max(length, 0);
		const startIndex = page * pageSize;
		const endIndex = startIndex < length ?
			Math.min(startIndex + pageSize, length) :
			startIndex + pageSize;

		if (length === Number.MAX_SAFE_INTEGER) {
			const many = $localize`:@@many:many`;
			return `${startIndex + 1} – ${endIndex} ${of} ${many}`;
		}

		return `${startIndex + 1} – ${endIndex} ${of} ${length}`;
	};
}
