import {
  ChangeDetectionStrategy,
  Component,
  ElementRef,
  inject,
  signal,
  viewChild,
} from '@angular/core';
import { PipelineDiagramComponent } from '../../components/pipeline-diagram/pipeline-diagram';
import { AnalysisEngine } from '../../services/analysis-engine';
import { DiagramHighlightService } from '../../services/diagram-highlight.service';
import { EVAL_LABELS, EvalMode, Trail } from '../../models/trail.model';

const PHASES = [
  'Krok 1/5 — normalizacja problemu…',
  'Krok 2/5 — formułowanie kontradykcji…',
  'Krok 3/5 — generowanie kandydatów…',
  'Krok 4/5 — ewaluacja…',
];
const PHASE_INTERVAL_MS = 8000;

const PHASE_NODES: string[][] = [
  ['problem_normalizer'],
  ['contradiction_extractor', 'triz_lookup', 'criteria_extractor'],
  ['candidate_generators', 'triz_generator', 'scamper_generator'],
  ['evaluator'],
];
const COMPLETION_NODES = ['choice_selector', 'trail_assembler'];

@Component({
  selector: 'app-analysis-page',
  imports: [PipelineDiagramComponent],
  templateUrl: './analysis-page.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AnalysisPage {
  private readonly engine = inject(AnalysisEngine);
  private readonly highlight = inject(DiagramHighlightService);
  private readonly resultsHeading =
    viewChild.required<ElementRef<HTMLHeadingElement>>('wynikiH');
  private readonly problemField =
    viewChild.required<ElementRef<HTMLTextAreaElement>>('problemField');

  protected readonly evalLabels = EVAL_LABELS;

  protected readonly problem = signal('');
  protected readonly evalMode = signal<EvalMode>('rubryka');
  protected readonly running = signal(false);
  protected readonly status = signal('');
  protected readonly error = signal('');
  protected readonly trail = signal<Trail | null>(null);

  protected async run(event: Event): Promise<void> {
    event.preventDefault();
    const problem = this.problem().trim();
    if (!problem) {
      this.error.set('Opisz problem w polu 01, zanim uruchomisz analizę.');
      this.problemField().nativeElement.focus();
      return;
    }

    this.error.set('');
    this.running.set(true);
    this.trail.set(null);

    this.highlight.reset();
    this.highlight.setActive(PHASE_NODES[0]);
    this.status.set(PHASES[0]);

    let phaseIdx = 1;
    const timer = setInterval(() => {
      if (phaseIdx < PHASES.length) {
        this.status.set(PHASES[phaseIdx]);
        const next = PHASE_NODES[phaseIdx];
        this.highlight.completePrevious(next);
        this.highlight.setActive(next);
        phaseIdx++;
      }
    }, PHASE_INTERVAL_MS);

    let success = false;
    try {
      const trail = await this.engine.solve(problem, this.evalMode());
      success = true;
      this.trail.set(trail);
      this.status.set('Analiza zakończona');
      this.highlight.completePrevious(COMPLETION_NODES);
      this.highlight.setActive(COMPLETION_NODES);
      setTimeout(() => {
        this.highlight.markDone(COMPLETION_NODES);
        this.resultsHeading().nativeElement.focus();
      }, 1500);
    } finally {
      clearInterval(timer);
      this.running.set(false);
      if (!success) this.highlight.reset();
    }
  }
}
