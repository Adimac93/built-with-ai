import {
  ChangeDetectionStrategy,
  Component,
  ElementRef,
  inject,
  signal,
  viewChild,
} from '@angular/core';
import { AnalysisEngine } from '../../services/analysis-engine';
import { EVAL_LABELS, EvalMode, Trail } from '../../models/trail.model';

const PHASES = [
  'Krok 1/5 — normalizacja problemu…',
  'Krok 2/5 — formułowanie kontradykcji…',
  'Krok 3/5 — generowanie kandydatów…',
  'Krok 4/5 — ewaluacja…',
];
const PHASE_INTERVAL_MS = 8000;

@Component({
  selector: 'app-analysis-page',
  templateUrl: './analysis-page.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AnalysisPage {
  private readonly engine = inject(AnalysisEngine);
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

    let phase = 0;
    this.status.set(PHASES[phase++]);
    const timer = setInterval(() => {
      if (phase < PHASES.length) this.status.set(PHASES[phase++]);
    }, PHASE_INTERVAL_MS);

    try {
      const trail = await this.engine.solve(problem, this.evalMode());
      this.trail.set(trail);
      this.status.set('Analiza zakończona');
      setTimeout(() => this.resultsHeading().nativeElement.focus());
    } catch {
      this.status.set('');
      this.error.set(
        'Nie udało się połączyć z silnikiem analizy. Sprawdź, czy API działa, i spróbuj ponownie.',
      );
    } finally {
      clearInterval(timer);
      this.running.set(false);
    }
  }
}
