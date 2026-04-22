export interface PagedResponse<T> {
	data: T[];
	totalItems: number;
	totalPages: number;
	currentPage: number;
	limit: number;
}
