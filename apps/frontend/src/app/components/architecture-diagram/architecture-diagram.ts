import { Component, effect, inject, untracked } from '@angular/core';
import {
  NgDiagramBackgroundComponent,
  NgDiagramComponent,
  NgDiagramMarkerComponent,
  NgDiagramNodeTemplateMap,
  NgDiagramService,
  NgDiagramViewportService,
  initializeModel,
  provideNgDiagram,
} from 'ng-diagram';
import { ArchitectureNodeComponent } from './architecture-node/architecture-node';

const WIDTH = 210;
const HEIGHT = 104;

function node(
  id: string,
  x: number,
  y: number,
  data: {
    readonly eyebrow: string;
    readonly title: string;
    readonly body: string;
    readonly kind: 'client' | 'app' | 'api' | 'agent' | 'cloud' | 'tooling' | 'data';
  },
) {
  return {
    id,
    type: 'architecture',
    position: { x, y },
    size: { width: WIDTH, height: HEIGHT },
    autoSize: false,
    data,
  };
}

function edge(
  id: string,
  source: string,
  target: string,
  sourcePort = 'port-right',
  targetPort = 'port-left',
) {
  return {
    id,
    source,
    target,
    sourcePort,
    targetPort,
    targetArrowhead: 'architecture-arrow',
    data: {},
  };
}

@Component({
  selector: 'app-architecture-diagram',
  imports: [NgDiagramComponent, NgDiagramBackgroundComponent, NgDiagramMarkerComponent],
  providers: [provideNgDiagram()],
  template: `
    <ng-diagram [model]="model" [nodeTemplateMap]="nodeTemplateMap">
      <ng-diagram-background />
      <ng-diagram-marker>
        <svg>
          <defs>
            <marker
              id="architecture-arrow"
              viewBox="0 0 10 6"
              refX="9"
              refY="3"
              markerWidth="10"
              markerHeight="6"
              orient="auto"
            >
              <path d="M0,0 L10,3 L0,6 Z" fill="context-stroke" />
            </marker>
          </defs>
        </svg>
      </ng-diagram-marker>
    </ng-diagram>
  `,
  styles: `
    :host {
      display: flex;
      height: min(72vh, 720px);
      min-height: 560px;
      border-top: 1px dashed var(--ds-color-border-muted);
      background: var(--ds-color-bg-canvas);
    }

    @media (max-width: 720px) {
      :host {
        height: 680px;
        min-height: 680px;
      }
    }
  `,
})
export class ArchitectureDiagramComponent {
  private readonly viewport = inject(NgDiagramViewportService);
  private readonly diagram = inject(NgDiagramService);
  private framed = false;

  constructor() {
    effect(() => {
      if (!this.diagram.isInitialized()) return;
      untracked(() => {
        if (this.framed) return;
        this.framed = true;
        this.viewport.zoomToFit({ padding: 44 });
      });
    });
  }

  readonly nodeTemplateMap = new NgDiagramNodeTemplateMap([
    ['architecture', ArchitectureNodeComponent],
  ]);

  readonly model = initializeModel({
    nodes: [
      node('user', 0, 220, {
        eyebrow: 'użytkownik',
        title: 'Problem',
        body: 'Wpisuje lub dyktuje problem techniczny w interfejsie Heureka.',
        kind: 'client',
      }),
      node('angular', 270, 110, {
        eyebrow: 'frontend',
        title: 'Angular 21',
        body: 'Standalone components, signals, routing, polski interfejs i kontrolki a11y.',
        kind: 'app',
      }),
      node('diagram', 270, 270, {
        eyebrow: 'wizualizacja',
        title: 'ng-diagram',
        body: 'Rysuje pipeline agenta i tę mapę architektury w przeglądarce.',
        kind: 'app',
      }),
      node('frontendRun', 540, 30, {
        eyebrow: 'gcp · cloud run',
        title: 'Frontend',
        body: 'Statyczna aplikacja Angular serwowana z kontenera Nginx.',
        kind: 'cloud',
      }),
      node('nestjs', 540, 190, {
        eyebrow: 'gateway',
        title: 'NestJS API',
        body: '/solve proxy do agenta, /speech/transcribe, Swagger pod /api/docs.',
        kind: 'api',
      }),
      node('speech', 810, 50, {
        eyebrow: 'google cloud',
        title: 'Speech-to-Text',
        body: 'Transkrybuje nagrania z mikrofonu po stronie serwera API.',
        kind: 'cloud',
      }),
      node('agentService', 810, 210, {
        eyebrow: 'gcp · cloud run',
        title: 'FastAPI Agent',
        body: 'Endpoint /solve, A2A routes, sesje ADK i telemetryka.',
        kind: 'agent',
      }),
      node('adk', 1080, 210, {
        eyebrow: 'google adk',
        title: 'SequentialAgent',
        body: 'Normalizacja, kontradykcja, lookup, generatory, ewaluacja i wybór.',
        kind: 'agent',
      }),
      node('vertex', 1350, 90, {
        eyebrow: 'vertex ai',
        title: 'Gemini',
        body: 'LLM odpowiada za kroki wymagające oceny i generowania treści.',
        kind: 'cloud',
      }),
      node('triz', 1350, 250, {
        eyebrow: 'deterministyczny kod',
        title: 'TRIZ + SCAMPER',
        body: 'Macierz 39x39, operatory SCAMPER, argmax wyboru i trail JSON.',
        kind: 'data',
      }),
      node('github', 270, 480, {
        eyebrow: 'ci/cd',
        title: 'GitHub Actions',
        body: 'Nx affected lint/test/build oraz deploy do Cloud Run.',
        kind: 'tooling',
      }),
      node('nx', 540, 480, {
        eyebrow: 'monorepo',
        title: 'Nx + pnpm',
        body: 'Trzy aplikacje w workspace: frontend, API i agent.',
        kind: 'tooling',
      }),
      node('docker', 810, 480, {
        eyebrow: 'artefakty',
        title: 'Docker',
        body: 'Obrazy dla Angular/Nginx, NestJS i agenta Python.',
        kind: 'tooling',
      }),
      node('terraform', 1080, 480, {
        eyebrow: 'infra as code',
        title: 'Terraform',
        body: 'Cloud Run, IAM, storage, telemetry outputs i public invoker.',
        kind: 'tooling',
      }),
      node('agentsCli', 1350, 480, {
        eyebrow: 'agent ops',
        title: 'agents-cli',
        body: 'Lokalny playground, eval, deploy i manifest agenta.',
        kind: 'tooling',
      }),
      node('gcp', 810, 640, {
        eyebrow: 'platforma',
        title: 'Google Cloud',
        body: 'Cloud Run, Vertex AI, Speech-to-Text, logging, trace i Workload Identity.',
        kind: 'cloud',
      }),
    ],
    edges: [
      edge('e-user-angular', 'user', 'angular'),
      edge('e-user-diagram', 'user', 'diagram'),
      edge('e-angular-api', 'angular', 'nestjs'),
      edge('e-diagram-angular', 'diagram', 'angular', 'port-top', 'port-bottom'),
      edge('e-frontend-api', 'frontendRun', 'nestjs', 'port-bottom', 'port-top'),
      edge('e-api-speech', 'nestjs', 'speech', 'port-right', 'port-left'),
      edge('e-api-agent', 'nestjs', 'agentService'),
      edge('e-agent-adk', 'agentService', 'adk'),
      edge('e-adk-vertex', 'adk', 'vertex', 'port-right', 'port-left'),
      edge('e-adk-triz', 'adk', 'triz', 'port-right', 'port-left'),
      edge('e-github-nx', 'github', 'nx'),
      edge('e-nx-docker', 'nx', 'docker'),
      edge('e-docker-gcp', 'docker', 'gcp', 'port-bottom', 'port-top'),
      edge('e-terraform-gcp', 'terraform', 'gcp', 'port-bottom', 'port-top'),
      edge('e-agentscli-agent', 'agentsCli', 'agentService', 'port-top', 'port-bottom'),
      edge('e-gcp-frontend', 'gcp', 'frontendRun', 'port-top', 'port-bottom'),
      edge('e-gcp-agent', 'gcp', 'agentService', 'port-top', 'port-bottom'),
    ],
  });
}
