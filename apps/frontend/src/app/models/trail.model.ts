export type GenerationMethod = 'triz' | 'scamper';
export type EvalMode = 'rubryka' | 'pugh' | 'pary';

export const METHOD_LABELS: Record<GenerationMethod, string> = {
  triz: 'TRIZ',
  scamper: 'SCAMPER',
};

export const EVAL_LABELS: Record<EvalMode, string> = {
  rubryka: 'Rubryka ważona',
  pugh: 'Macierz Pugha',
  pary: 'Porównanie parami',
};

export interface CriterionScore {
  readonly criterion: string;
  readonly score: number;
}

export interface TrailCandidate {
  readonly method: GenerationMethod;
  readonly source: string;
  readonly name: string;
  readonly description: string;
  readonly score: number;
  /** Oceny per kryterium z kroku 4 — argumentacja wyboru. */
  readonly breakdown: readonly CriterionScore[];
}

export interface TrailParameter {
  readonly tag: string;
  readonly name: string;
  readonly paramNumber: number;
}

export interface TrailContradiction {
  readonly better: TrailParameter;
  readonly worse: TrailParameter;
  readonly principles: string;
}

export interface MethodGroup {
  readonly method: GenerationMethod;
  readonly label: string;
  readonly candidates: readonly TrailCandidate[];
}

export interface Trail {
  readonly problem: string;
  readonly evalMode: EvalMode;
  readonly contradiction: TrailContradiction;
  readonly groups: readonly MethodGroup[];
  readonly ranked: readonly TrailCandidate[];
  readonly winner: TrailCandidate;
  /** Drugi w rankingu — do pokazania przewagi zwycięzcy. */
  readonly runnerUp: TrailCandidate | null;
  readonly candidateCount: number;
}
