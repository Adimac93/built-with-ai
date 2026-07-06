//! Prompt texts ported verbatim from the Python ADK agents (`app/agent.py`).
//! ADK interpolated session state into `{placeholders}`; here the values are
//! rendered directly into the final prompt string.

use serde_json::Value;

pub const TRIZ_PARAMETERS_TEXT: &str = "\
1-Weight of moving object, 2-Weight of stationary object, 3-Length of moving object,
4-Length of stationary object, 5-Area of moving object, 6-Area of stationary object,
7-Volume of moving object, 8-Volume of stationary object, 9-Speed, 10-Force,
11-Stress or pressure, 12-Shape, 13-Stability of composition, 14-Strength,
15-Duration of action (moving), 16-Duration of action (stationary), 17-Temperature,
18-Illumination intensity, 19-Use of energy (moving), 20-Use of energy (stationary),
21-Power, 22-Loss of energy, 23-Loss of substance, 24-Loss of information,
25-Loss of time, 26-Quantity of substance, 27-Reliability, 28-Measurement accuracy,
29-Manufacturing precision, 30-Object-generated harmful effects, 31-Harmful side effects,
32-Ease of manufacture, 33-Ease of operation, 34-Ease of repair, 35-Adaptability,
36-Device complexity, 37-Difficulty of detecting, 38-Extent of automation, 39-Productivity";

/// (letter, name, hint) — defined by the system, not the LLM.
pub const SCAMPER_OPERATORS: [(&str, &str, &str); 7] = [
    (
        "S",
        "Substitute",
        "What materials, processes, or components can be substituted?",
    ),
    (
        "C",
        "Combine",
        "What elements or ideas can be merged or combined?",
    ),
    (
        "A",
        "Adapt",
        "What can be adapted or borrowed from other domains?",
    ),
    (
        "M",
        "Modify/Magnify/Minimize",
        "What can be modified, magnified, minimized, or rearranged?",
    ),
    (
        "P",
        "Put to other uses",
        "How can existing elements be used for different purposes?",
    ),
    (
        "E",
        "Eliminate",
        "What can be removed, simplified, or reduced to its core?",
    ),
    (
        "R",
        "Reverse/Rearrange",
        "What can be reversed, inverted, or reordered?",
    ),
];

/// Step 1 system instruction; the raw user problem goes in the user turn.
pub const NORMALIZER_INSTRUCTION: &str = "\
You are a technical problem analyst. Analyze the inventive problem and
output a normalized problem statement that includes:
- Core challenge (1 sentence)
- Domain context
- Key constraints mentioned
- Desired outcome

Be concise. Do not add information not in the original problem.";

pub fn contradiction_prompt(problem: &str) -> String {
    format!(
        "You are a TRIZ expert. Extract the core technical contradiction from this problem:

PROBLEM: {problem}

A technical contradiction exists when improving one engineering parameter degrades another.
Identify which parameter we want to IMPROVE and which one WORSENS as a side effect.
Both values MUST be integers between 1 and 39 inclusive.

TRIZ engineering parameters:
{TRIZ_PARAMETERS_TEXT}

Return JSON with improving_param (int 1-39), worsening_param (int 1-39), and justification (string).
"
    )
}

pub fn criteria_prompt(problem: &str) -> String {
    format!(
        "You are a solution evaluator. Extract evaluation criteria from the problem.

PROBLEM: {problem}

Generic criteria (always include all three):
1. \"Resolves the technical contradiction\"
2. \"Technical feasibility\"
3. \"Cost and scalability\"

Problem-specific criteria: derive 2-4 constraints explicitly stated or strongly implied by the problem.
Example: if problem says \"keep transport as-is\", add \"Transport continuity preserved\".

Return JSON with generic_criteria (list of 3 strings) and problem_specific_criteria (list of 2-4 strings)."
    )
}

pub fn triz_generator_prompt(
    problem: &str,
    improving_name: &str,
    worsening_name: &str,
    principles_text: &str,
) -> String {
    format!(
        "You are a TRIZ solution inventor. Generate solution candidates using TRIZ inventive principles.

PROBLEM: {problem}
CONTRADICTION: Improving \"{improving_name}\" while \"{worsening_name}\" worsens.
TRIZ PRINCIPLES TO APPLY: {principles_text}

For EACH principle listed above, generate ONE concrete, specific solution idea.
Each idea must:
- Directly address the core problem
- Be traceable to its principle (set trace_id to \"triz-<principle_number>\")
- Be feasible (not science fiction)

Return JSON with a \"candidates\" list. Each candidate: principle_number (int), principle_name (str), idea (str), trace_id (str)."
    )
}

pub fn scamper_prompt(problem: &str) -> String {
    let operators_text = SCAMPER_OPERATORS
        .iter()
        .map(|(op, name, hint)| format!("- {op} ({name}): {hint}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "You are a creative problem solver using the SCAMPER method.

PROBLEM: {problem}

Apply each of the 7 SCAMPER operators below to generate solution ideas.
You MUST generate at least one candidate for S, C, A, M, P, E, R (select the most promising ones).
Return at least 3 candidates total.

SCAMPER OPERATORS (defined by the system — do not change or omit):
{operators_text}

For each candidate: operator (single letter), operator_name (str), idea (str), trace_id = \"scamper-<letter>\".
Return JSON with a \"candidates\" list."
    )
}

pub fn evaluator_prompt(
    problem: &str,
    criteria: &Value,
    triz_candidates: &Value,
    scamper_candidates: &Value,
) -> String {
    format!(
        "You are an objective solution evaluator. Score each candidate solution.

PROBLEM: {problem}

EVALUATION CRITERIA: {criteria}

ALL TRIZ CANDIDATES: {triz_candidates}
ALL SCAMPER CANDIDATES: {scamper_candidates}

For EVERY candidate (TRIZ + SCAMPER), score each criterion 0-100 and sum to total.
Use trace_id to identify candidates (e.g. \"triz-35\", \"scamper-S\").

Return JSON with an \"evaluations\" list. Each item: trace_id (str), scores (dict: criterion→score int), justification (str), total (int = sum of all scores)."
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn triz_prompt_inlines_lookup_values() {
        let prompt = triz_generator_prompt(
            "problem text",
            "Power",
            "Temperature",
            "35: Parameter changes; 10: Preliminary action",
        );
        assert!(prompt.contains("Improving \"Power\" while \"Temperature\" worsens"));
        assert!(prompt.contains("35: Parameter changes; 10: Preliminary action"));
    }

    #[test]
    fn scamper_prompt_lists_all_seven_operators() {
        let prompt = scamper_prompt("p");
        for (op, name, _) in SCAMPER_OPERATORS {
            assert!(prompt.contains(&format!("- {op} ({name})")));
        }
    }

    #[test]
    fn evaluator_prompt_embeds_candidates_json() {
        let prompt = evaluator_prompt(
            "p",
            &json!({"generic_criteria": ["a"]}),
            &json!({"candidates": [{"trace_id": "triz-35"}]}),
            &json!({"candidates": [{"trace_id": "scamper-S"}]}),
        );
        assert!(prompt.contains("\"triz-35\""));
        assert!(prompt.contains("\"scamper-S\""));
    }
}
