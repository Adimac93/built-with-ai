"""LLM-as-judge for the inventive problem solving trail.

Evaluates structural completeness (code) + semantic quality (LLM).
Wired in from eval_config.yaml via custom_function_file.
"""

import json
import re

from google import genai
from google.genai import types
from pydantic import BaseModel


class _Verdict(BaseModel):
    score: int  # 1-5
    explanation: str


def _extract_trail(response: str) -> dict | None:
    """Extract the JSON trail from the agent's final response text."""
    if not response:
        return None
    # Try direct JSON parse
    try:
        data = json.loads(response)
        if isinstance(data, dict) and "step1_problem" in data:
            return data
    except json.JSONDecodeError:
        pass
    # Try to find JSON block in response text
    match = re.search(r"\{.*\"step1_problem\".*\}", response, re.DOTALL)
    if match:
        try:
            return json.loads(match.group(0))
        except json.JSONDecodeError:
            pass
    return None


def _structural_score(trail: dict | None) -> dict:
    """Deterministic structural checks — code-based, no LLM.

    Returns score 0-5 based on how many required trail components are present.
    """
    if trail is None:
        return {"structural_score": 0, "issues": ["No trail JSON found in response"]}

    issues = []
    score = 5

    if not trail.get("step1_problem"):
        issues.append("Missing step1_problem")
        score -= 1

    contradiction = trail.get("step2_contradiction") or {}
    improving = contradiction.get("improving_param", 0)
    worsening = contradiction.get("worsening_param", 0)
    if not (1 <= int(improving) <= 39 and 1 <= int(worsening) <= 39):
        issues.append(f"Contradiction params out of range: {improving}, {worsening}")
        score -= 1

    lookup = trail.get("step2a_lookup") or {}
    if not lookup.get("principles"):
        issues.append("TRIZ lookup returned no principles")
        score -= 1

    triz = (trail.get("step3a_triz_candidates") or {}).get("candidates", [])
    scamper = (trail.get("step3b_scamper_candidates") or {}).get("candidates", [])
    if len(triz) < 3:
        issues.append(f"TRIZ candidates < 3 (got {len(triz)})")
        score -= 1
    if len(scamper) < 3:
        issues.append(f"SCAMPER candidates < 3 (got {len(scamper)})")
        score -= 1

    # Check traceability
    for c in triz:
        tid = c.get("trace_id", "")
        if not tid.startswith("triz-"):
            issues.append(f"TRIZ candidate missing trace_id: {tid}")

    evaluation = (trail.get("step4_evaluation") or {}).get("evaluations", [])
    all_candidate_ids = {c.get("trace_id") for c in triz + scamper}
    eval_ids = {e.get("trace_id") for e in evaluation}
    missing = all_candidate_ids - eval_ids
    if missing:
        issues.append(f"Candidates not evaluated: {missing}")

    choice = trail.get("step5_choice") or {}
    winner_id = choice.get("winner_id", "")
    if not winner_id or winner_id == "none":
        issues.append("No winner selected")
        score = max(0, score - 1)
    else:
        # Verify argmax: winner should have highest total
        if evaluation:
            best = max(evaluation, key=lambda e: int(e.get("total", 0)))
            if best.get("trace_id") != winner_id:
                issues.append(
                    f"Choice {winner_id} is not argmax (best is {best['trace_id']})"
                )
                score = max(0, score - 1)

    return {"structural_score": max(0, score), "issues": issues}


def evaluate(instance):
    response = instance.get("response", "")
    trail = _extract_trail(response)
    structural = _structural_score(trail)

    reference = instance.get("reference", "")
    prompt = (
        "You are an expert evaluator for an Inventive Problem Solving AI system. "
        "Grade the agent's response on a 1-5 scale (1=poor, 5=excellent) for: "
        "quality of the contradiction formulation, creativity and feasibility of candidates, "
        "and whether the evaluation criteria make sense for the problem.\n"
        f"User Prompt: {instance.get('prompt', '')}\n"
        f"Agent Response (trail JSON): {response[:3000]}\n"
    )
    if reference:
        prompt += f"Expected behavior: {reference}\n"
    prompt += (
        f"Structural check result: {structural}\n"
        "Focus on semantic quality — structural issues are already flagged above."
    )

    client = genai.Client()
    resp = client.models.generate_content(
        model="gemini-flash-latest",
        contents=prompt,
        config=types.GenerateContentConfig(
            temperature=0,
            response_mime_type="application/json",
            response_schema=_Verdict,
        ),
    )
    verdict = resp.parsed
    if verdict is None:
        semantic_score = 0
        explanation = resp.text or ""
    else:
        semantic_score = max(1, min(5, verdict.score))
        explanation = verdict.explanation

    combined = (structural["structural_score"] + semantic_score) / 2
    return {
        "score": round(combined, 2),
        "structural_score": structural["structural_score"],
        "structural_issues": structural["issues"],
        "semantic_score": semantic_score,
        "explanation": explanation,
    }
