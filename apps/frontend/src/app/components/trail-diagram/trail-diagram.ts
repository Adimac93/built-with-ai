import {
  ChangeDetectionStrategy,
  Component,
  Injector,
  computed,
  inject,
  input,
} from '@angular/core';
import {
  NgDiagramComponent,
  NgDiagramNodeTemplateMap,
  NgDiagramViewportService,
  initializeModel,
  provideNgDiagram,
} from 'ng-diagram';
import type { Edge, ModelAdapter, Node } from 'ng-diagram';
import { EVAL_LABELS, Trail } from '../../models/trail.model';
import { TrailNode, TrailNodeData } from './trail-node';

const COL = {
  problem: 0,
  contradiction: 260,
  candidates: 540,
  evaluation: 830,
  choice: 1090,
};
const ROW_H = 96;

@Component({
  selector: 'app-trail-diagram',
  imports: [NgDiagramComponent],
  providers: [provideNgDiagram()],
  template: `<ng-diagram
    [model]="model()"
    [nodeTemplateMap]="templateMap"
    (diagramInit)="fitView()"
  />`,
  styles: `
    :host {
      display: flex;
      height: 100%;
    }
  `,
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class TrailDiagram {
  public readonly trail = input.required<Trail>();

  private readonly injector = inject(Injector);
  private readonly viewport = inject(NgDiagramViewportService);

  protected fitView(): void {
    this.viewport.zoomToFit({ padding: 32 });
  }

  protected readonly templateMap = new NgDiagramNodeTemplateMap([
    ['trail', TrailNode],
  ]);

  protected readonly model = computed<ModelAdapter>(() =>
    initializeModel(this.buildModel(this.trail()), this.injector),
  );

  private buildModel(trail: Trail): { nodes: Node[]; edges: Edge[] } {
    const candidateCount = trail.ranked.length;
    const midY = ((candidateCount - 1) * ROW_H) / 2;

    const node = (
      id: string,
      x: number,
      y: number,
      data: TrailNodeData,
    ): Node => ({
      id,
      type: 'trail',
      position: { x, y },
      data,
    });

    const edge = (id: string, source: string, target: string): Edge => ({
      id,
      source,
      sourcePort: 'out',
      target,
      targetPort: 'in',
      data: {},
    });

    const nodes: Node[] = [
      node('problem', COL.problem, midY, {
        tag: 'Krok 1',
        label: 'Problem',
        kind: 'stage',
      }),
      node('evaluation', COL.evaluation, midY, {
        tag: `Krok 4 · ${EVAL_LABELS[trail.evalMode]}`,
        label: `Ewaluacja ${candidateCount} kandydatów`,
        kind: 'stage',
      }),
      node('choice', COL.choice, midY, {
        tag: `Krok 5 · argmax · ${trail.winner.score}/100`,
        label: `Wybrano: ${trail.winner.name}`,
        kind: 'winner',
      }),
    ];
    const edges: Edge[] = [edge('e-choice', 'evaluation', 'choice')];

    // Dwie niezależne gałęzie generowania: TRIZ przechodzi przez kontradykcję
    // i matrycę, SCAMPER działa operatorami wprost na problemie.
    let row = 0;
    for (const group of trail.groups) {
      const hubId = `hub-${group.method}`;
      const clusterMidY = (row + (group.candidates.length - 1) / 2) * ROW_H;

      nodes.push(
        node(
          hubId,
          COL.contradiction,
          clusterMidY,
          group.method === 'triz'
            ? {
                tag: 'Krok 2 · kontradykcja → matryca',
                label: `${trail.contradiction.better.name} × ${trail.contradiction.worse.name}`,
                kind: 'stage',
              }
            : {
                tag: 'Krok 3 · operatory na problemie',
                label: group.label,
                kind: 'stage',
              },
        ),
      );
      edges.push(edge(`e-problem-${hubId}`, 'problem', hubId));

      for (const c of group.candidates) {
        nodes.push(
          node(c.name, COL.candidates, row * ROW_H, {
            tag: `${group.label} · ${c.source}`,
            label: c.name,
            kind: 'candidate',
          }),
        );
        edges.push(
          edge(`e-gen-${c.name}`, hubId, c.name),
          edge(`e-eval-${c.name}`, c.name, 'evaluation'),
        );
        row++;
      }
    }

    return { nodes, edges };
  }
}
