import { Component, input } from '@angular/core';
import {
  NgDiagramNodeSelectedDirective,
  NgDiagramPortComponent,
  type NgDiagramNodeTemplate,
  type Node,
} from 'ng-diagram';

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
    <div class="agent-node" [class]="'agent-node--' + node().data.agentType.toLowerCase()">
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
      font-family: 'IBM Plex Mono', monospace;
      border: 1.5px solid var(--ink, #1b2b21);
      background: var(--card, #fafbf6);
      padding: 10px 12px;
      width: 180px;
      box-sizing: border-box;
    }

    .agent-node--baseagent {
      border-color: var(--graphite, #5a685a);
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
      color: var(--graphite, #5a685a);
    }

    .agent-node__type {
      font-size: 0.58rem;
      letter-spacing: 0.1em;
      text-transform: uppercase;
      color: var(--graphite, #5a685a);
      background: var(--paper, #f1f4ec);
      padding: 1px 5px;
    }

    .agent-node--baseagent .agent-node__type {
      color: var(--red, #c93a26);
    }

    .agent-node__name {
      display: block;
      font-size: 0.78rem;
      font-weight: 600;
      color: var(--ink, #1b2b21);
      word-break: break-all;
      margin-bottom: 6px;
    }

    .agent-node__prompt summary {
      font-size: 0.6rem;
      letter-spacing: 0.12em;
      text-transform: uppercase;
      color: var(--graphite, #5a685a);
      cursor: pointer;
      list-style: none;
      padding: 3px 0;
      border-top: 1px dashed #8f9c8c;
    }
    .agent-node__prompt summary::-webkit-details-marker { display: none; }
    .agent-node__prompt summary::before { content: '▶ '; font-size: 0.55em; }
    .agent-node__prompt[open] summary::before { content: '▼ '; }

    .agent-node__prompt-text {
      margin-top: 5px;
      font-size: 0.6rem;
      line-height: 1.5;
      color: var(--ink, #1b2b21);
      white-space: pre-wrap;
      word-break: break-word;
      background: var(--paper, #f1f4ec);
      border: 1px solid #8f9c8c;
      padding: 6px 8px;
      max-height: 160px;
      overflow-y: auto;
    }
  `,
})
export class AgentNodeComponent implements NgDiagramNodeTemplate<AgentNodeData> {
  node = input.required<Node<AgentNodeData>>();
}
