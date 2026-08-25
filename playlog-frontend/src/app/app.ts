import {Component, ChangeDetectionStrategy} from '@angular/core';
import {RouterOutlet} from '@angular/router';
import {Navbar} from './features/navbar/navbar';

@Component({
	selector: 'app-root',
	imports: [RouterOutlet, Navbar],
	templateUrl: './app.html',
	styleUrl: './app.css',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class App {
}
