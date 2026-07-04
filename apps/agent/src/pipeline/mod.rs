pub mod prompts;
pub mod schemas;

use serde_json::{json, Value};

use crate::error::AgentError;
use crate::gemini::{GeminiClient, Output};
use crate::triz;

/// Runs the full TRIZ+SCAMPER pipeline and assembles the reasoning trail.
/// Mirrors the Python `SequentialAgent` execution order:
/// normalize → contradiction → matrix lookup → criteria →
/// (TRIZ ∥ SCAMPER candidates) → evaluate → argmax choice → trail.
pub async fn run(gemini: &GeminiClient, raw_problem: &str) -> Result<Value, AgentError> {
    // Step 1 — problem normalizer (plain text).
    let problem = gemini
        .generate_text(Some(prompts::NORMALIZER_INSTRUCTION), raw_problem, Output::Text)
        .await?;

    // Step 2 — contradiction extractor (structured).
    let contradiction: schemas::ContradictionResult = gemini
        .generate_json(
            &prompts::contradiction_prompt(&problem),
            Some(&schemas::contradiction_schema()),
        )
        .await?;

    // Step 2a — deterministic matrix lookup; out-of-range params degrade to
    // an empty principle list instead of failing (Python caught ValueError).
    let (lookup, principles_text) =
        build_lookup(contradiction.improving_param, contradiction.worsening_param);
    tracing::info!(
        "TRIZ lookup: {} → {}, principles: {principles_text}",
        lookup["improving_name"], lookup["worsening_name"]
    );

    // Step 1a — criteria extractor (structured).
    let criteria: schemas::EvaluationCriteria = gemini
        .generate_json(&prompts::criteria_prompt(&problem), Some(&schemas::criteria_schema()))
        .await?;
    let criteria = serde_json::to_value(criteria).expect("criteria serializes");

    // Steps 3a ∥ 3b — candidate generators run in parallel (ADK ParallelAgent).
    let triz_prompt = prompts::triz_generator_prompt(
        &problem,
        lookup["improving_name"].as_str().unwrap_or_default(),
        lookup["worsening_name"].as_str().unwrap_or_default(),
        &principles_text,
    );
    let scamper_prompt = prompts::scamper_prompt(&problem);
    let triz_schema = schemas::triz_candidates_schema();
    let scamper_schema = schemas::scamper_candidates_schema();
    let (triz_candidates, scamper_candidates) = tokio::join!(
        gemini.generate_json::<schemas::TrizCandidates>(&triz_prompt, Some(&triz_schema)),
        gemini.generate_json::<schemas::ScamperCandidates>(&scamper_prompt, Some(&scamper_schema)),
    );
    let triz_candidates = serde_json::to_value(triz_candidates?).expect("candidates serialize");
    let scamper_candidates =
        serde_json::to_value(scamper_candidates?).expect("candidates serialize");

    // Step 4 — evaluator (JSON mode; open `scores` map has no schema).
    let evaluation: schemas::EvaluationResult = gemini
        .generate_json(
            &prompts::evaluator_prompt(&problem, &criteria, &triz_candidates, &scamper_candidates),
            None,
        )
        .await?;

    // Step 5 — deterministic argmax choice.
    let choice = select_winner(&evaluation);
    tracing::info!("Choice: {} (score: {})", choice["winner_id"], choice["winner_total"]);

    // Trail assembly — exact key set the frontend consumes.
    Ok(json!({
        "step1_problem": problem,
        "step2_contradiction": contradiction,
        "step2a_lookup": lookup,
        "step1a_criteria": criteria,
        "step3a_triz_candidates": triz_candidates,
        "step3b_scamper_candidates": scamper_candidates,
        "step4_evaluation": evaluation,
        "step5_choice": choice,
    }))
}

/// Builds the `step2a_lookup` object plus the `"n: name; …"` text the TRIZ
/// generator prompt embeds (Python's flat `lookup_principles_text` state key).
fn build_lookup(improving: i64, worsening: i64) -> (Value, String) {
    let principles = triz::lookup_principles(improving, worsening).unwrap_or_else(|err| {
        tracing::error!("TRIZ lookup failed: {err}");
        Vec::new()
    });

    let principle_dicts: Vec<Value> =
        principles.iter().map(|&p| triz::get_principle_description(p)).collect();
    let principles_text = if principle_dicts.is_empty() {
        "No principles found in matrix for this pair".to_string()
    } else {
        principle_dicts
            .iter()
            .map(|p| {
                format!(
                    "{}: {}",
                    p["number"].as_str().unwrap_or_default(),
                    p["name"].as_str().unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };

    let lookup = json!({
        "improving_param": improving,
        "improving_name": triz::get_parameter_name(improving),
        "worsening_param": worsening,
        "worsening_name": triz::get_parameter_name(worsening),
        "principles": principle_dicts,
        "principles_count": principles.len(),
    });
    (lookup, principles_text)
}

/// Argmax over evaluation totals; mirrors Python `ChoiceAgent` exactly,
/// including the empty-evaluations fallback.
fn select_winner(evaluation: &schemas::EvaluationResult) -> Value {
    match evaluation.evaluations.iter().max_by_key(|e| e.total) {
        None => json!({
            "winner_id": "none",
            "winner_total": 0,
            "reasoning": "No evaluations available",
        }),
        Some(winner) => json!({
            "winner_id": winner.trace_id,
            "winner_total": winner.total,
            "reasoning": format!(
                "Highest aggregate score ({}) across all criteria. argmax selection.",
                winner.total
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::schemas::{CandidateScore, EvaluationResult};
    use std::collections::BTreeMap;

    fn score(trace_id: &str, total: i64) -> CandidateScore {
        CandidateScore {
            trace_id: trace_id.into(),
            scores: BTreeMap::new(),
            justification: String::new(),
            total,
        }
    }

    #[test]
    fn winner_is_argmax_of_totals() {
        let evaluation = EvaluationResult {
            evaluations: vec![score("triz-2", 505), score("scamper-A", 525), score("scamper-S", 475)],
        };
        let choice = select_winner(&evaluation);
        assert_eq!(choice["winner_id"], "scamper-A");
        assert_eq!(choice["winner_total"], 525);
        assert_eq!(
            choice["reasoning"],
            "Highest aggregate score (525) across all criteria. argmax selection."
        );
    }

    #[test]
    fn empty_evaluations_falls_back_to_none() {
        let choice = select_winner(&EvaluationResult { evaluations: vec![] });
        assert_eq!(choice["winner_id"], "none");
        assert_eq!(choice["winner_total"], 0);
        assert_eq!(choice["reasoning"], "No evaluations available");
    }

    #[test]
    fn lookup_produces_string_numbers_and_text() {
        // Power (21) improving vs Temperature (17) worsening — known pair.
        let (lookup, text) = build_lookup(21, 17);
        assert_eq!(lookup["improving_name"], "Power");
        assert_eq!(lookup["worsening_name"], "Temperature");
        let principles = lookup["principles"].as_array().unwrap();
        assert_eq!(lookup["principles_count"], principles.len());
        assert!(!principles.is_empty());
        assert!(principles.iter().all(|p| p["number"].is_string()));
        assert!(text.contains(": "));
    }

    #[test]
    fn lookup_degrades_gracefully_on_invalid_params() {
        let (lookup, text) = build_lookup(0, 99);
        assert_eq!(lookup["principles_count"], 0);
        assert_eq!(lookup["improving_name"], "Parameter 0");
        assert_eq!(text, "No principles found in matrix for this pair");
    }
}
