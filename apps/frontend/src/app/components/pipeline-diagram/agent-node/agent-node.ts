import { Component, computed, inject, input } from '@angular/core';
import {
  NgDiagramNodeSelectedDirective,
  NgDiagramPortComponent,
  type NgDiagramNodeTemplate,
  type Node,
} from 'ng-diagram';
import { DiagramHighlightService } from '../../../services/diagram-highlight.service';

export interface AgentNodeData {
  name: string;
  agentType: 'LlmAgent' | 'BaseAgent';
  step: string;
  prompt: string;
  isDeterministic: boolean;
}

@Component({
  selector: 'app-agent-node',
  imports: [NgDiagramPortComponent],
  hostDirectives: [{ directive: NgDiagramNodeSelectedDirective, inputs: ['node'] }],
  template: `
    <div class="agent-node"
      [class]="'agent-node--' + node().data.agentType.toLowerCase()"
      [class.agent-node--active]="highlight() === 'active'"
      [class.agent-node--done]="highlight() === 'done'">
      <div class="agent-node__header">
        <span class="agent-node__step">{{ node().data.step }}</span>
        <span class="agent-node__type">{{ node().data.agentType }}</span>
      </div>
      <span class="agent-node__name">{{ node().data.name }}</span>
      <details class="agent-node__prompt" data-no-pan="true" data-no-drag="true">
        <summary>{{ node().data.isDeterministic ? 'opis logiki' : 'prompt systemowy' }}</summary>
        <pre class="agent-node__prompt-text">{{ node().data.prompt }}</pre>
      </details>
    </div>
    <ng-diagram-port side="left" type="both" id="port-left" />
    <ng-diagram-port side="right" type="both" id="port-right" />
  `,
  styles: `
    :host { display: block; position: relative; }

    .agent-node {
      font-family: var(--ds-font-family-code, 'IBM Plex Mono', monospace);
      border: var(--ds-border-default);
      background: var(--ds-color-bg-card);
      padding: 10px 12px;
      width: 180px;
      box-sizing: border-box;
    }

    .agent-node--baseagent {
      border-color: var(--ds-color-content-secondary);
    }

    .agent-node--active {
      border-color: var(--ds-component-diagram-active-border) !important;
      background: color-mix(
        in srgb,
        var(--ds-component-diagram-active-bg) 15%,
        var(--ds-color-bg-card)
      );
      animation: node-pulse 1.4s ease-in-out infinite;
    }

    .agent-node--done {
      border-color: var(--ds-color-content-secondary);
      opacity: 0.55;
    }

    @keyframes node-pulse {
      0%, 100% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--ds-component-diagram-active-shadow) 0%, transparent); }
      50%       { box-shadow: 0 0 0 4px color-mix(in srgb, var(--ds-component-diagram-active-shadow) 28%, transparent); }
    }

    .agent-node__header {
      display: flex;
      justify-content: space-between;
      align-items: baseline;
      gap: 6px;
      margin-bottom: 4px;
    }

    .agent-node__step {
      font-size: 0.58rem;
      letter-spacing: 0.14em;
      text-transform: uppercase;
      color: var(--ds-color-content-secondary);
    }

    .agent-node__type {
      font-size: 0.58rem;
      letter-spacing: 0.1em;
      text-transform: uppercase;
      color: var(--ds-color-content-secondary);
      background: var(--ds-color-bg-canvas);
      padding: 1px 5px;
    }

    .agent-node--baseagent .agent-node__type {
      color: var(--ds-color-accent-danger);
    }

    .agent-node__name {
      display: block;
      font-size: 0.78rem;
      font-weight: 600;
      color: var(--ds-color-content-primary);
      word-break: break-all;
      margin-bottom: 6px;
    }

    .agent-node__prompt summary {
      font-size: 0.6rem;
      letter-spacing: 0.12em;
      text-transform: uppercase;
      color: var(--ds-color-content-secondary);
      cursor: pointer;
      list-style: none;
      padding: 3px 0;
      border-top: var(--ds-border-muted-dashed);
    }
    .agent-node__prompt summary::-webkit-details-marker { display: none; }
    .agent-node__prompt summary::before { content: '▶ '; font-size: 0.55em; }
    .agent-node__prompt[open] summary::before { content: '▼ '; }

    .agent-node__prompt-text {
      margin-top: 5px;
      font-size: 0.6rem;
      line-height: 1.5;
      color: var(--ds-color-content-primary);
      white-space: pre-wrap;
      word-break: break-word;
      background: var(--ds-color-bg-canvas);
      border: var(--ds-border-muted);
      padding: 6px 8px;
      max-height: 160px;
      overflow-y: auto;
    }
  `,
})
export class AgentNodeComponent implements NgDiagramNodeTemplate<AgentNodeData> {
  node = input.required<Node<AgentNodeData>>();

  private readonly highlightService = inject(DiagramHighlightService);
  protected readonly highlight = computed(() => this.highlightService.highlightOf(this.node().id));
}
