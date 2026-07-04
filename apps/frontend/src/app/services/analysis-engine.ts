import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { firstValueFrom } from 'rxjs';
import { ConfigProvider } from '../config/config-provider';
import {
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

    const trizCandidates: TrailCandidate[] = trizRaw.map((c, i) => ({
      method: 'triz' as const,
      source: `Zasada ${c.principle_number} · ${c.principle_name}`,
      name: `Kandydat T${i + 1}`,
      description: c.idea,
      score: scoreOf.get(c.trace_id) ?? 0,
    }));

    const scamperCandidates: TrailCandidate[] = scamperRaw.map((c, i) => ({
      method: 'scamper' as const,
      source: `Operator ${c.operator} · ${c.operator_name}`,
      name: `Kandydat S${i + 1}`,
      description: c.idea,
      score: scoreOf.get(c.trace_id) ?? 0,
    }));

    const allCandidates = [...trizCandidates, ...scamperCandidates];
    const ranked = [...allCandidates].sort((a, b) => b.score - a.score);

    const winnerId = r.step5_choice?.winner_id;
    const traceEntries = [
      ...trizRaw.map((c, i) => ({ traceId: c.trace_id, candidate: trizCandidates[i] })),
      ...scamperRaw.map((c, i) => ({ traceId: c.trace_id, candidate: scamperCandidates[i] })),
    ];
    const winner = traceEntries.find((x) => x.traceId === winnerId)?.candidate ?? ranked[0];

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
      candidateCount: allCandidates.length,
    };
  }
}
