import { ChangeDetectionStrategy, Component } from '@angular/core';
import { ArchitectureDiagramComponent } from '../../components/architecture-diagram/architecture-diagram';

@Component({
  selector: 'app-architecture-page',
  imports: [ArchitectureDiagramComponent],
  templateUrl: './architecture-page.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ArchitecturePage {}
