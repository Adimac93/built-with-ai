import { Component, computed, inject, input } from '@angular/core';
import {
  NgDiagramGroupHighlightedDirective,
  NgDiagramNodeSelectedDirective,
  NgDiagramPortComponent,
  type GroupNode,
  type NgDiagramGroupNodeTemplate,
} from 'ng-diagram';
import { DiagramHighlightService } from '../../../services/diagram-highlight.service';

export interface ParallelGroupData {
  name: string;
  step: string;
}

@Component({
  selector: 'app-parallel-group',
  imports: [NgDiagramPortComponent, NgDiagramGroupHighlightedDirective],
  hostDirectives: [{ directive: NgDiagramNodeSelectedDirective, inputs: ['node'] }],
  template: `
    <div class="par-group" ngDiagramGroupHighlighted [node]="node()"
      [class.par-group--active]="highlight() === 'active'"
      [class.par-group--done]="highlight() === 'done'">
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
      font-family: var(--ds-font-family-code, 'IBM Plex Mono', monospace);
      border: 1.5px dashed var(--ds-color-border-strong);
      background: color-mix(in srgb, var(--ds-color-bg-canvas) 60%, transparent);
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
      color: var(--ds-color-content-secondary);
    }

    .par-group__type {
      font-size: 0.58rem;
      letter-spacing: 0.1em;
      text-transform: uppercase;
      color: var(--ds-color-content-primary);
      background: var(--ds-color-bg-canvas);
      padding: 1px 5px;
    }

    .par-group__name {
      display: block;
      font-size: 0.78rem;
      font-weight: 600;
      color: var(--ds-color-content-primary);
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .par-group--active {
      border-color: var(--ds-component-diagram-active-border) !important;
      background: color-mix(in srgb, var(--ds-component-diagram-active-bg) 12%, transparent);
      animation: group-pulse 1.4s ease-in-out infinite;
    }

    .par-group--done {
      border-color: var(--ds-color-content-secondary);
      opacity: 0.55;
    }

    @keyframes group-pulse {
      0%, 100% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--ds-component-diagram-active-shadow) 0%, transparent); }
      50%       { box-shadow: 0 0 0 5px color-mix(in srgb, var(--ds-component-diagram-active-shadow) 22%, transparent); }
    }
  `,
})
export class ParallelGroupComponent implements NgDiagramGroupNodeTemplate<ParallelGroupData> {
  node = input.required<GroupNode<ParallelGroupData>>();

  private readonly highlightService = inject(DiagramHighlightService);
  protected readonly highlight = computed(() => this.highlightService.highlightOf(this.node().id));
}
