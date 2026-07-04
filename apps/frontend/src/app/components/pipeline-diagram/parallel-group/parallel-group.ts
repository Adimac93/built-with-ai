import { Component, input } from '@angular/core';
import {
  NgDiagramGroupHighlightedDirective,
  NgDiagramNodeSelectedDirective,
  NgDiagramPortComponent,
  type GroupNode,
  type NgDiagramGroupNodeTemplate,
} from 'ng-diagram';

export interface ParallelGroupData {
  name: string;
  step: string;
}

@Component({
  selector: 'app-parallel-group',
  imports: [NgDiagramPortComponent, NgDiagramGroupHighlightedDirective],
  hostDirectives: [{ directive: NgDiagramNodeSelectedDirective, inputs: ['node'] }],
  template: `
    <div class="par-group" ngDiagramGroupHighlighted [node]="node()">
      <div class="par-group__header">
        <span class="par-group__step">{{ node().data.step }}</span>
        <span class="par-group__type">ParallelAgent</span>
      </div>
      <span class="par-group__name">{{ node().data.name }}</span>
    </div>
    <ng-diagram-port side="left" type="both" id="port-left" />
    <ng-diagram-port side="right" type="both" id="port-right" />
  `,
  styles: `
    :host { display: block; position: relative; width: 100%; height: 100%; }

    .par-group {
      font-family: 'IBM Plex Mono', monospace;
      border: 1.5px dashed var(--ink, #1b2b21);
      background: color-mix(in srgb, var(--paper, #f1f4ec) 60%, transparent);
      padding: 10px 12px;
      width: 100%;
      height: 100%;
      box-sizing: border-box;
    }

    .par-group__header {
      display: flex;
      justify-content: space-between;
      align-items: baseline;
      gap: 6px;
      margin-bottom: 4px;
    }

    .par-group__step {
      font-size: 0.58rem;
      letter-spacing: 0.14em;
      text-transform: uppercase;
      color: var(--graphite, #5a685a);
    }

    .par-group__type {
      font-size: 0.58rem;
      letter-spacing: 0.1em;
      text-transform: uppercase;
      color: var(--ink, #1b2b21);
      background: var(--paper, #f1f4ec);
      padding: 1px 5px;
    }

    .par-group__name {
      display: block;
      font-size: 0.78rem;
      font-weight: 600;
      color: var(--ink, #1b2b21);
    }
  `,
})
export class ParallelGroupComponent implements NgDiagramGroupNodeTemplate<ParallelGroupData> {
  node = input.required<GroupNode<ParallelGroupData>>();
}
