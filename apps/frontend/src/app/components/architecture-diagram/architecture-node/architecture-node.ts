import { Component, input } from '@angular/core';
import {
  NgDiagramNodeSelectedDirective,
  NgDiagramPortComponent,
  type NgDiagramNodeTemplate,
  type Node,
} from 'ng-diagram';

export interface ArchitectureNodeData {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly kind: 'client' | 'app' | 'api' | 'agent' | 'cloud' | 'tooling' | 'data';
}

@Component({
  selector: 'app-architecture-node',
  imports: [NgDiagramPortComponent],
  hostDirectives: [{ directive: NgDiagramNodeSelectedDirective, inputs: ['node'] }],
  template: `
    <article class="arch-node" [class]="'arch-node--' + node().data.kind">
      <p class="arch-node__eyebrow">{{ node().data.eyebrow }}</p>
      <h2 class="arch-node__title">{{ node().data.title }}</h2>
      <p class="arch-node__body">{{ node().data.body }}</p>
    </article>

    <ng-diagram-port side="left" type="both" id="port-left" />
    <ng-diagram-port side="right" type="both" id="port-right" />
    <ng-diagram-port side="top" type="both" id="port-top" />
    <ng-diagram-port side="bottom" type="both" id="port-bottom" />
  `,
  styles: `
    :host {
      display: block;
      position: relative;
    }

    .arch-node {
      display: flex;
      flex-direction: column;
      justify-content: center;
      width: 210px;
      min-height: 104px;
      padding: 11px 13px;
      border: var(--ds-border-default);
      background: var(--ds-color-bg-card);
      color: var(--ds-color-content-primary);
      font-family: var(--ds-font-family-body);
      box-sizing: border-box;
    }

    .arch-node--client {
      border-color: var(--ds-color-accent-danger);
    }

    .arch-node--api,
    .arch-node--agent {
      background: color-mix(in srgb, var(--ds-color-bg-card) 88%, var(--ds-color-primitive-amber-300));
    }

    .arch-node--cloud {
      background: var(--ds-color-bg-selected);
      color: var(--ds-color-content-inverse);
      border-color: var(--ds-color-bg-selected);
    }

    .arch-node--tooling {
      border-style: dashed;
      background: var(--ds-color-bg-canvas);
    }

    .arch-node--data {
      background: color-mix(in srgb, var(--ds-color-bg-card) 82%, var(--ds-color-primitive-red-100));
    }

    .arch-node__eyebrow {
      margin-bottom: 4px;
      font-family: var(--ds-font-family-code);
      font-size: 0.62rem;
      letter-spacing: 0.14em;
      line-height: 1.3;
      text-transform: uppercase;
      color: currentColor;
      opacity: 0.72;
    }

    .arch-node__title {
      margin: 0;
      font-family: var(--ds-font-family-display);
      font-size: 1.24rem;
      line-height: 1;
      letter-spacing: 0;
      text-transform: uppercase;
    }

    .arch-node__body {
      margin-top: 7px;
      font-size: 0.76rem;
      line-height: 1.34;
      color: currentColor;
      opacity: 0.86;
    }
  `,
})
export class ArchitectureNodeComponent
  implements NgDiagramNodeTemplate<ArchitectureNodeData>
{
  readonly node = input.required<Node<ArchitectureNodeData>>();
}
