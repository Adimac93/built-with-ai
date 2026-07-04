//! Serde models and Gemini `responseSchema` constants mirroring the pydantic
//! models the ADK agents used as `output_schema` (`app/schemas.py`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContradictionResult {
    pub improving_param: i64,
    pub worsening_param: i64,
    pub justification: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    pub generic_criteria: Vec<String>,
    pub problem_specific_criteria: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrizCandidate {
    pub principle_number: i64,
    pub principle_name: String,
    pub idea: String,
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrizCandidates {
    pub candidates: Vec<TrizCandidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScamperCandidate {
    pub operator: String,
    pub operator_name: String,
    pub idea: String,
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScamperCandidates {
    pub candidates: Vec<ScamperCandidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandidateScore {
    pub trace_id: String,
    /// criterion → score 0-100. BTreeMap keeps serialization order stable.
    pub scores: BTreeMap<String, i64>,
    pub justification: String,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub evaluations: Vec<CandidateScore>,
}

pub fn contradiction_schema() -> Value {
    json!({
        "type": "OBJECT",
        "properties": {
            "improving_param": { "type": "INTEGER", "description": "TRIZ parameter number to improve (1-39)" },
            "worsening_param": { "type": "INTEGER", "description": "TRIZ parameter number that worsens (1-39)" },
            "justification": { "type": "STRING", "description": "Why this contradiction captures the core tension" }
        },
        "required": ["improving_param", "worsening_param", "justification"]
    })
}

pub fn criteria_schema() -> Value {
    json!({
        "type": "OBJECT",
        "properties": {
            "generic_criteria": { "type": "ARRAY", "items": { "type": "STRING" } },
            "problem_specific_criteria": { "type": "ARRAY", "items": { "type": "STRING" } }
        },
        "required": ["generic_criteria", "problem_specific_criteria"]
    })
}

pub fn triz_candidates_schema() -> Value {
    json!({
        "type": "OBJECT",
        "properties": {
            "candidates": {
                "type": "ARRAY",
                "items": {
                    "type": "OBJECT",
                    "properties": {
                        "principle_number": { "type": "INTEGER" },
                        "principle_name": { "type": "STRING" },
                        "idea": { "type": "STRING" },
                        "trace_id": { "type": "STRING" }
                    },
                    "required": ["principle_number", "principle_name", "idea", "trace_id"]
                }
            }
        },
        "required": ["candidates"]
    })
}

pub fn scamper_candidates_schema() -> Value {
    json!({
        "type": "OBJECT",
        "properties": {
            "candidates": {
                "type": "ARRAY",
                "items": {
                    "type": "OBJECT",
                    "properties": {
                        "operator": { "type": "STRING" },
                        "operator_name": { "type": "STRING" },
                        "idea": { "type": "STRING" },
                        "trace_id": { "type": "STRING" }
                    },
                    "required": ["operator", "operator_name", "idea", "trace_id"]
                }
            }
        },
        "required": ["candidates"]
    })
}

// Note: no evaluator schema — `scores` is an open criterion→int map, which the
// Gemini responseSchema OpenAPI subset cannot express (no additionalProperties).
// The evaluator call uses responseMimeType JSON only; the prompt fully
// specifies the shape, and parsing into `EvaluationResult` validates it.
