//! Domain model + mapping from the `/solve` API response to the rendered
//! reasoning trail. Pure logic — ported from Angular's `analysis-engine.ts`
//! and `trail.model.ts`, unit-tested on the host target.

use serde::Deserialize;

pub const METHOD_LABEL_TRIZ: &str = "TRIZ";
pub const METHOD_LABEL_SCAMPER: &str = "SCAMPER";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EvalMode {
    Rubryka,
    Pugh,
    Pary,
}

impl EvalMode {
    pub fn label(self) -> &'static str {
        match self {
            EvalMode::Rubryka => "Rubryka ważona",
            EvalMode::Pugh => "Macierz Pugha",
            EvalMode::Pary => "Porównanie parami",
        }
    }
}

// ── Raw API shapes (tolerant: every level optional/defaulted) ──

#[derive(Deserialize, Default)]
pub struct SolveApiResponse {
    #[serde(default)]
    pub step1_problem: String,
    #[serde(default)]
    pub step2a_lookup: Lookup,
    #[serde(default)]
    pub step3a_triz_candidates: TrizCandidates,
    #[serde(default)]
    pub step3b_scamper_candidates: ScamperCandidates,
    #[serde(default)]
    pub step4_evaluation: Evaluation,
    #[serde(default)]
    pub step5_choice: Choice,
}

#[derive(Deserialize, Default)]
pub struct Lookup {
    #[serde(default)]
    pub improving_param: Option<i64>,
    #[serde(default)]
    pub improving_name: String,
    #[serde(default)]
    pub worsening_param: Option<i64>,
    #[serde(default)]
    pub worsening_name: String,
    #[serde(default)]
    pub principles: Vec<Principle>,
}

#[derive(Deserialize, Default)]
pub struct Principle {
    #[serde(default)]
    pub number: String,
    /// Deserialized for completeness; the UI joins only the numbers.
    #[serde(default)]
    #[allow(dead_code)]
    pub name: String,
}

#[derive(Deserialize, Default)]
pub struct TrizCandidates {
    #[serde(default)]
    pub candidates: Vec<TrizCandidate>,
}

#[derive(Deserialize, Default)]
pub struct TrizCandidate {
    #[serde(default)]
    pub principle_number: i64,
    #[serde(default)]
    pub principle_name: String,
    #[serde(default)]
    pub idea: String,
    #[serde(default)]
    pub trace_id: String,
}

#[derive(Deserialize, Default)]
pub struct ScamperCandidates {
    #[serde(default)]
    pub candidates: Vec<ScamperCandidate>,
}

#[derive(Deserialize, Default)]
pub struct ScamperCandidate {
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub operator_name: String,
    #[serde(default)]
    pub idea: String,
    #[serde(default)]
    pub trace_id: String,
}

#[derive(Deserialize, Default)]
pub struct Evaluation {
    #[serde(default)]
    pub evaluations: Vec<CandidateScore>,
}

#[derive(Deserialize, Default)]
pub struct CandidateScore {
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub scores: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub total: i64,
}

#[derive(Deserialize, Default)]
pub struct Choice {
    #[serde(default)]
    pub winner_id: String,
}

// ── Rendered trail model ──

#[derive(Clone, PartialEq, Debug)]
pub struct CriterionScore {
    pub criterion: String,
    pub score: i64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TrailCandidate {
    pub source: String,
    pub name: String,
    pub description: String,
    pub score: i64,
    pub breakdown: Vec<CriterionScore>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TrailParameter {
    pub tag: String,
    pub name: String,
    pub param_number: i64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TrailContradiction {
    pub better: TrailParameter,
    pub worse: TrailParameter,
    pub principles: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MethodGroup {
    pub label: &'static str,
    pub candidates: Vec<TrailCandidate>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Trail {
    pub problem: String,
    pub eval_mode: EvalMode,
    pub contradiction: TrailContradiction,
    pub groups: Vec<MethodGroup>,
    pub ranked: Vec<TrailCandidate>,
    pub winner: TrailCandidate,
    pub runner_up: Option<TrailCandidate>,
    pub candidate_count: usize,
}

/// Only the top 3 per method (by total score) enter the trail.
const TOP_PER_METHOD: usize = 3;

/// Maps the raw API response to the rendered trail — same semantics as the
/// Angular `mapToTrail`: top-3 per method, ranked union, winner matched by
/// `winner_id` with a fallback to the top-ranked candidate.
pub fn map_to_trail(r: &SolveApiResponse, eval_mode: EvalMode) -> Option<Trail> {
    let score_of = |trace_id: &str| -> i64 {
        r.step4_evaluation
            .evaluations
            .iter()
            .find(|e| e.trace_id == trace_id)
            .map(|e| e.total)
            .unwrap_or(0)
    };
    let breakdown_of = |trace_id: &str| -> Vec<CriterionScore> {
        r.step4_evaluation
            .evaluations
            .iter()
            .find(|e| e.trace_id == trace_id)
            .map(|e| {
                e.scores
                    .iter()
                    .map(|(criterion, score)| CriterionScore {
                        criterion: criterion.replace('_', " "),
                        score: score.as_i64().unwrap_or(0),
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    fn top_of(mut trace_ids: Vec<&str>, score_of: impl Fn(&str) -> i64) -> Vec<&str> {
        trace_ids.sort_by_key(|id| std::cmp::Reverse(score_of(id)));
        trace_ids.truncate(TOP_PER_METHOD);
        trace_ids
    }

    let triz_top = top_of(
        r.step3a_triz_candidates
            .candidates
            .iter()
            .map(|c| c.trace_id.as_str())
            .collect(),
        score_of,
    );
    let scamper_top = top_of(
        r.step3b_scamper_candidates
            .candidates
            .iter()
            .map(|c| c.trace_id.as_str())
            .collect(),
        score_of,
    );

    let triz_candidates: Vec<TrailCandidate> = triz_top
        .iter()
        .enumerate()
        .filter_map(|(i, id)| {
            let c = r
                .step3a_triz_candidates
                .candidates
                .iter()
                .find(|c| c.trace_id == *id)?;
            Some(TrailCandidate {
                source: format!("Zasada {} · {}", c.principle_number, c.principle_name),
                name: format!("Kandydat T{}", i + 1),
                description: c.idea.clone(),
                score: score_of(id),
                breakdown: breakdown_of(id),
            })
        })
        .collect();

    let scamper_candidates: Vec<TrailCandidate> = scamper_top
        .iter()
        .enumerate()
        .filter_map(|(i, id)| {
            let c = r
                .step3b_scamper_candidates
                .candidates
                .iter()
                .find(|c| c.trace_id == *id)?;
            Some(TrailCandidate {
                source: format!("Operator {} · {}", c.operator, c.operator_name),
                name: format!("Kandydat S{}", i + 1),
                description: c.idea.clone(),
                score: score_of(id),
                breakdown: breakdown_of(id),
            })
        })
        .collect();

    let all: Vec<TrailCandidate> = triz_candidates
        .iter()
        .chain(scamper_candidates.iter())
        .cloned()
        .collect();
    let mut ranked = all.clone();
    ranked.sort_by_key(|c| std::cmp::Reverse(c.score));

    let winner_id = r.step5_choice.winner_id.as_str();
    let winner = triz_top
        .iter()
        .position(|id| *id == winner_id)
        .and_then(|i| triz_candidates.get(i))
        .or_else(|| {
            scamper_top
                .iter()
                .position(|id| *id == winner_id)
                .and_then(|i| scamper_candidates.get(i))
        })
        .or_else(|| ranked.first())?
        .clone();
    let runner_up = ranked.iter().find(|c| **c != winner).cloned();

    let lookup = &r.step2a_lookup;
    let principle_nums = lookup
        .principles
        .iter()
        .map(|p| p.number.as_str())
        .collect::<Vec<_>>()
        .join(" · ");

    let param = |number: Option<i64>, name: &str| TrailParameter {
        tag: match number {
            Some(n) => format!("Parametr {n}"),
            None => "Parametr ?".to_string(),
        },
        name: name.to_string(),
        param_number: number.unwrap_or(0),
    };

    Some(Trail {
        problem: r.step1_problem.clone(),
        eval_mode,
        contradiction: TrailContradiction {
            better: param(lookup.improving_param, &lookup.improving_name),
            worse: param(lookup.worsening_param, &lookup.worsening_name),
            principles: principle_nums,
        },
        groups: vec![
            MethodGroup {
                label: METHOD_LABEL_TRIZ,
                candidates: triz_candidates,
            },
            MethodGroup {
                label: METHOD_LABEL_SCAMPER,
                candidates: scamper_candidates,
            },
        ],
        candidate_count: all.len(),
        ranked,
        winner,
        runner_up,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn response(json: serde_json::Value) -> SolveApiResponse {
        serde_json::from_value(json).unwrap()
    }

    fn full_response() -> SolveApiResponse {
        response(json!({
            "step1_problem": "Znormalizowany problem",
            "step2a_lookup": {
                "improving_param": 21, "improving_name": "Power",
                "worsening_param": 17, "worsening_name": "Temperature",
                "principles": [{"number": "35", "name": "Parameter changes"}, {"number": "2", "name": "Taking out"}],
            },
            "step3a_triz_candidates": {"candidates": [
                {"principle_number": 35, "principle_name": "Parameter changes", "idea": "i35", "trace_id": "triz-35"},
                {"principle_number": 2, "principle_name": "Taking out", "idea": "i2", "trace_id": "triz-2"},
                {"principle_number": 14, "principle_name": "Spheroidality", "idea": "i14", "trace_id": "triz-14"},
                {"principle_number": 17, "principle_name": "Another dimension", "idea": "i17", "trace_id": "triz-17"},
            ]},
            "step3b_scamper_candidates": {"candidates": [
                {"operator": "S", "operator_name": "Substitute", "idea": "sS", "trace_id": "scamper-S"},
                {"operator": "C", "operator_name": "Combine", "idea": "sC", "trace_id": "scamper-C"},
            ]},
            "step4_evaluation": {"evaluations": [
                {"trace_id": "triz-35", "scores": {"Technical_feasibility": 80}, "total": 400},
                {"trace_id": "triz-2", "scores": {}, "total": 520},
                {"trace_id": "triz-14", "scores": {}, "total": 300},
                {"trace_id": "triz-17", "scores": {}, "total": 450},
                {"trace_id": "scamper-S", "scores": {}, "total": 510},
                {"trace_id": "scamper-C", "scores": {}, "total": 200},
            ]},
            "step5_choice": {"winner_id": "triz-2", "winner_total": 520},
        }))
    }

    #[test]
    fn keeps_top_three_per_method_sorted_by_score() {
        let trail = map_to_trail(&full_response(), EvalMode::Rubryka).unwrap();
        let triz = &trail.groups[0].candidates;
        assert_eq!(triz.len(), 3);
        // triz-2 (520) > triz-17 (450) > triz-35 (400); triz-14 (300) dropped.
        assert_eq!(triz[0].score, 520);
        assert_eq!(triz[0].name, "Kandydat T1");
        assert_eq!(triz[0].source, "Zasada 2 · Taking out");
        assert_eq!(triz[2].score, 400);
        assert_eq!(trail.groups[1].candidates.len(), 2);
        assert_eq!(trail.candidate_count, 5);
    }

    #[test]
    fn ranked_unions_both_methods_descending() {
        let trail = map_to_trail(&full_response(), EvalMode::Rubryka).unwrap();
        let scores: Vec<i64> = trail.ranked.iter().map(|c| c.score).collect();
        assert_eq!(scores, vec![520, 510, 450, 400, 200]);
    }

    #[test]
    fn winner_matched_by_trace_id() {
        let trail = map_to_trail(&full_response(), EvalMode::Rubryka).unwrap();
        assert_eq!(trail.winner.source, "Zasada 2 · Taking out");
        assert_eq!(trail.winner.score, 520);
        let runner_up = trail.runner_up.unwrap();
        assert_eq!(runner_up.score, 510);
    }

    #[test]
    fn winner_outside_top_three_falls_back_to_ranked_first() {
        let mut r = full_response();
        // triz-14 was dropped from top-3, so winner_id no longer matches.
        r.step5_choice.winner_id = "triz-14".into();
        let trail = map_to_trail(&r, EvalMode::Rubryka).unwrap();
        assert_eq!(trail.winner.score, 520);
    }

    #[test]
    fn breakdown_replaces_underscores_in_criteria() {
        let trail = map_to_trail(&full_response(), EvalMode::Rubryka).unwrap();
        let t35 = trail.groups[0]
            .candidates
            .iter()
            .find(|c| c.score == 400)
            .unwrap();
        assert_eq!(t35.breakdown[0].criterion, "Technical feasibility");
        assert_eq!(t35.breakdown[0].score, 80);
    }

    #[test]
    fn principles_joined_with_middle_dots() {
        let trail = map_to_trail(&full_response(), EvalMode::Rubryka).unwrap();
        assert_eq!(trail.contradiction.principles, "35 · 2");
        assert_eq!(trail.contradiction.better.tag, "Parametr 21");
        assert_eq!(trail.contradiction.worse.param_number, 17);
    }

    #[test]
    fn empty_response_yields_no_trail() {
        assert!(map_to_trail(&SolveApiResponse::default(), EvalMode::Rubryka).is_none());
    }

    #[test]
    fn tolerates_missing_sections() {
        let r = response(json!({ "step1_problem": "p", "step3a_triz_candidates": {"candidates": [
            {"principle_number": 1, "principle_name": "Segmentation", "idea": "i", "trace_id": "triz-1"},
        ]}}));
        let trail = map_to_trail(&r, EvalMode::Pugh).unwrap();
        assert_eq!(trail.winner.score, 0);
        assert_eq!(trail.contradiction.better.tag, "Parametr ?");
        assert!(trail.runner_up.is_none());
    }
}
