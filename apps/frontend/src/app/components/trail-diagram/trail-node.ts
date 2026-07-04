import { ChangeDetectionStrategy, Component, input } from '@angular/core';
import {
  NgDiagramNodeTemplate,
  NgDiagramPortComponent,
  SimpleNode,
} from 'ng-diagram';

export interface TrailNodeData {
  readonly tag: string;
  readonly label: string;
  readonly kind: 'stage' | 'candidate' | 'winner';
  [key: string]: unknown;
}

@Component({
  selector: 'app-trail-node',
  imports: [NgDiagramPortComponent],
  template: `
    <div
      class="tnode"
      [class.tnode--candidate]="node().data.kind === 'candidate'"
      [class.tnode--winner]="node().data.kind === 'winner'"
    >
      <span class="tnode__tag">{{ node().data.tag }}</span>
      <p class="tnode__label">{{ node().data.label }}</p>
      <ng-diagram-port id="in" side="left" type="target" />
      <ng-diagram-port id="out" side="right" type="source" />
    </div>
  `,
  styles: `
    .tnode {
      background: var(--card, #fafbf6);
      border: 1.5px solid var(--ink, #1b2b21);
      padding: 10px 14px;
      min-width: 170px;
      max-width: 210px;
      font-family: 'IBM Plex Sans', sans-serif;
      color: var(--ink, #1b2b21);
    }
    .tnode--candidate {
      border-width: 1px;
      border-color: var(--line, #8f9c8c);
    }
    .tnode--winner {
      border-color: var(--red, #c93a26);
      box-shadow: 0 0 0 3px rgba(201, 58, 38, 0.15);
    }
    .tnode__tag {
      display: block;
      font-family: 'IBM Plex Mono', monospace;
      font-size: 0.62rem;
      letter-spacing: 0.14em;
      text-transform: uppercase;
      color: var(--graphite, #5a685a);
      margin-bottom: 2px;
    }
    .tnode--winner .tnode__tag {
      color: var(--red, #c93a26);
    }
    .tnode__label {
      margin: 0;
      font-weight: 600;
      font-size: 0.85rem;
      line-height: 1.3;
    }
  `,
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class TrailNode implements NgDiagramNodeTemplate<TrailNodeData> {
  public readonly node = input.required<SimpleNode<TrailNodeData>>();
}
