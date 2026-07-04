import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { firstValueFrom } from 'rxjs';
import { ConfigProvider } from '../config/config-provider';
import {
  CriterionScore,
  EvalMode,
  METHOD_LABELS,
  MethodGroup,
  Trail,
  TrailCandidate,
} from '../models/trail.model';

interface TrizCandidate {
  principle_number: number;
  principle_name: string;
  idea: string;
  trace_id: string;
}

interface ScamperCandidate {
  operator: string;
  operator_name: string;
  idea: string;
  trace_id: string;
}

interface CandidateScore {
  trace_id: string;
  scores: Record<string, number>;
  total: number;
}

interface SolveApiResponse {
  step1_problem: string;
  step2_contradiction: {
    improving_param: number;
    worsening_param: number;
    justification: string;
  };
  step2a_lookup: {
    improving_param: number;
    improving_name: string;
    worsening_param: number;
    worsening_name: string;
    principles: Array<{ number: string; name: string }>;
  };
  step3a_triz_candidates: { candidates: TrizCandidate[] };
  step3b_scamper_candidates: { candidates: ScamperCandidate[] };
  step4_evaluation: { evaluations: CandidateScore[] };
  step5_choice: { winner_id: string; winner_total: number };
}

@Injectable({ providedIn: 'root' })
export class AnalysisEngine {
  private readonly http = inject(HttpClient);
  private readonly config = inject(ConfigProvider);

  async solve(problem: string, evalMode: EvalMode): Promise<Trail> {
    const url = `${this.config.apiUrl()}/solve`;
    const response = await firstValueFrom(
      this.http.post<SolveApiResponse>(url, { problem }),
    );
    return this.mapToTrail(response, evalMode);
  }

  private mapToTrail(r: SolveApiResponse, evalMode: EvalMode): Trail {
    const lookup = r.step2a_lookup ?? {};
    const trizRaw: TrizCandidate[] = r.step3a_triz_candidates?.candidates ?? [];
    const scamperRaw: ScamperCandidate[] = r.step3b_scamper_candidates?.candidates ?? [];
    const evaluations: CandidateScore[] = r.step4_evaluation?.evaluations ?? [];

    const scoreOf = new Map<string, number>(
      evaluations.map((e) => [e.trace_id, e.total]),
    );
    const breakdownOf = new Map<string, CriterionScore[]>(
      evaluations.map((e) => [
        e.trace_id,
        Object.entries(e.scores ?? {}).map(([criterion, score]) => ({
          criterion: criterion.split('_').join(' '),
          score,
        })),
      ]),
    );

    // Top 3 per metoda (wg wyniku łącznego) — tylko ci wchodzą do traila.
    const TOP_PER_METHOD = 3;

    const topOf = <T extends { trace_id: string }>(raw: T[]): T[] =>
      [...raw]
        .sort(
          (a, b) =>
            (scoreOf.get(b.trace_id) ?? 0) - (scoreOf.get(a.trace_id) ?? 0),
        )
        .slice(0, TOP_PER_METHOD);

    const trizTop = topOf(trizRaw);
    const scamperTop = topOf(scamperRaw);

    const trizCandidates: TrailCandidate[] = trizTop.map((c, i) => ({
      method: 'triz' as const,
      source: `Zasada ${c.principle_number} · ${c.principle_name}`,
      name: `Kandydat T${i + 1}`,
      description: c.idea,
      score: scoreOf.get(c.trace_id) ?? 0,
      breakdown: breakdownOf.get(c.trace_id) ?? [],
    }));

    const scamperCandidates: TrailCandidate[] = scamperTop.map((c, i) => ({
      method: 'scamper' as const,
      source: `Operator ${c.operator} · ${c.operator_name}`,
      name: `Kandydat S${i + 1}`,
      description: c.idea,
      score: scoreOf.get(c.trace_id) ?? 0,
      breakdown: breakdownOf.get(c.trace_id) ?? [],
    }));

    const allCandidates = [...trizCandidates, ...scamperCandidates];
    const ranked = [...allCandidates].sort((a, b) => b.score - a.score);

    const winnerId = r.step5_choice?.winner_id;
    const traceEntries = [
      ...trizTop.map((c, i) => ({ traceId: c.trace_id, candidate: trizCandidates[i] })),
      ...scamperTop.map((c, i) => ({ traceId: c.trace_id, candidate: scamperCandidates[i] })),
    ];
    const winner = traceEntries.find((x) => x.traceId === winnerId)?.candidate ?? ranked[0];
    const runnerUp = ranked.find((c) => c !== winner) ?? null;

    const principleNums = (lookup.principles ?? [])
      .map((p) => p.number)
      .join(' · ');

    const groups: MethodGroup[] = [
      { method: 'triz', label: METHOD_LABELS['triz'], candidates: trizCandidates },
      { method: 'scamper', label: METHOD_LABELS['scamper'], candidates: scamperCandidates },
    ];

    return {
      problem: r.step1_problem ?? '',
      evalMode,
      contradiction: {
        better: {
          tag: `Parametr ${lookup.improving_param ?? '?'}`,
          name: lookup.improving_name ?? '',
          paramNumber: lookup.improving_param ?? 0,
        },
        worse: {
          tag: `Parametr ${lookup.worsening_param ?? '?'}`,
          name: lookup.worsening_name ?? '',
          paramNumber: lookup.worsening_param ?? 0,
        },
        principles: principleNums,
      },
      groups,
      ranked,
      winner,
      runnerUp,
      candidateCount: allCandidates.length,
    };
  }
}
