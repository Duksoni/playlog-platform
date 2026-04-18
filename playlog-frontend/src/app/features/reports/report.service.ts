import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {
	CreateReportRequest,
	ReportResponse,
	UpdateReportStatusRequest,
} from './report.dto';

@Injectable({
	providedIn: 'root',
})
export class ReportService {
	private http = inject(HttpClient);
	private readonly base = `${environment.apiUrl}/reports`;

	report(body: CreateReportRequest) {
		return this.http.post<ReportResponse>(this.base, body);
	}

	getPendingReports(page?: number) {
		let params = new HttpParams();
		if (page !== undefined) params = params.set('page', (page + 1).toString());
		return this.http.get<ReportResponse[]>(`${this.base}/pending`, {params});
	}

	resolveReport(id: string, body: UpdateReportStatusRequest) {
		return this.http.put<void>(`${this.base}/${id}/status`, body);
	}
}
