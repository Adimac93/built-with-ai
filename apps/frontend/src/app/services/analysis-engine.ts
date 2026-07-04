import { Injectable } from '@angular/core';
import {
  EvalMode,
  GenerationMethod,
  METHOD_LABELS,
  Trail,
  TrailCandidate,
} from '../models/trail.model';

const MOCK_CANDIDATES: Record<
  GenerationMethod,
  ReadonlyArray<readonly [source: string, description: string]>
> = {
  triz: [
    [
      'Zasada 1 · Segmentacja',
      'Podziel kluczowy obiekt lub proces na niezależne części, tak aby zmiana lub awaria jednej nie przenosiła się na całość.',
    ],
    [
      'Zasada 13 · Odwrócenie',
      'Wykonaj działanie odwrotnie: zamień elementy ruchome z nieruchomymi albo odwróć kolejność operacji w procesie.',
    ],
    [
      'Zasada 35 · Zmiana parametrów',
      'Zmień stan, gęstość, elastyczność lub temperaturę kluczowego elementu, aby osłabić źródło sprzeczności.',
    ],
  ],
  scamper: [
    [
      'Operator S · Substitute',
      'Zastąp najbardziej zawodny element procesu innym materiałem, mechanizmem lub etapem o tej samej funkcji.',
    ],
    [
      'Operator C · Combine',
      'Połącz dwie istniejące funkcje lub urządzenia w jedno rozwiązanie, które eliminuje słabe ogniwo.',
    ],
    [
      'Operator A · Adapt',
      'Zaadaptuj sprawdzone rozwiązanie z innej branży lub kontekstu do warunków zgłoszonego problemu.',
    ],
  ],
};

const CANDIDATE_PREFIX: Record<GenerationMethod, string> = {
  triz: 'T',
  scamper: 'S',
};

const MOCK_SCORES = [84, 71, 77, 63, 69, 58] as const;
const MOCK_DELAY_MS = 1800;

/**
 * Silnik analizy. Zgodnie z zadaniem jeden przebieg generuje kandydatów
 * dwiema metodami — TRIZ (matryca kontradykcji) + SCAMPER — a ewaluacja
 * i wybór obejmują wszystkich kandydatów razem.
 * Na razie zwraca dane przykładowe (mock) — docelowo ten serwis woła
 * REST API NestJS; komponenty nie wymagają wtedy żadnych zmian.
 */
@Injectable({ providedIn: 'root' })
export class AnalysisEngine {
  solve(problem: string, evalMode: EvalMode): Promise<Trail> {
    const methods: readonly GenerationMethod[] = ['triz', 'scamper'];
    const candidates: TrailCandidate[] = methods.flatMap((method, m) =>
      MOCK_CANDIDATES[method].map(([source, description], i) => ({
        method,
        source,
        description,
        name: `Kandydat ${CANDIDATE_PREFIX[method]}${i + 1}`,
        score: MOCK_SCORES[(m * 3 + i) % MOCK_SCORES.length],
      })),
    );
    const ranked = [...candidates].sort((a, b) => b.score - a.score);

    const trail: Trail = {
      problem,
      evalMode,
      contradiction: {
        better: { tag: 'Parametr 27', name: 'Niezawodność' },
        worse: { tag: 'Parametr 36', name: 'Złożoność systemu' },
        principles: '1 · 13 · 35',
      },
      groups: methods.map((method) => ({
        method,
        label: METHOD_LABELS[method],
        candidates: candidates.filter((c) => c.method === method),
      })),
      ranked,
      winner: ranked[0],
      candidateCount: candidates.length,
    };

    return new Promise((resolve) =>
      setTimeout(() => resolve(trail), MOCK_DELAY_MS),
    );
  }
}
