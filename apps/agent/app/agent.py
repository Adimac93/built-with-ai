# ruff: noqa
import json
import logging
from collections.abc import AsyncGenerator

from google.adk.agents import Agent, BaseAgent, ParallelAgent, SequentialAgent
from google.adk.agents.invocation_context import InvocationContext
from google.adk.apps import App
from google.adk.events import Event
from google.genai import types as genai_types

from .schemas import (
    ContradictionResult,
    EvaluationCriteria,
    EvaluationResult,
    ScamperCandidates,
    TrizCandidates,
)
from .triz import (
    get_parameter_name,
    get_principle_description,
    lookup_principles,
)

logger = logging.getLogger(__name__)

_TRIZ_PARAMETERS_TEXT = """
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
36-Device complexity, 37-Difficulty of detecting, 38-Extent of automation, 39-Productivity
""".strip()

_SCAMPER_OPERATORS = [
    ("S", "Substitute", "What materials, processes, or components can be substituted?"),
    ("C", "Combine", "What elements or ideas can be merged or combined?"),
    ("A", "Adapt", "What can be adapted or borrowed from other domains?"),
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
    ("E", "Eliminate", "What can be removed, simplified, or reduced to its core?"),
    ("R", "Reverse/Rearrange", "What can be reversed, inverted, or reordered?"),
]

# ── Step 1: Problem normalizer ────────────────────────────────────────────────


def create_problem_normalizer() -> Agent:
    return Agent(
        name="problem_normalizer",
        model="gemini-flash-latest",
        instruction="""You are a technical problem analyst. Analyze the inventive problem and
output a normalized problem statement that includes:
- Core challenge (1 sentence)
- Domain context
- Key constraints mentioned
- Desired outcome

Be concise. Do not add information not in the original problem.""",
        output_key="problem",
    )


# ── Step 2: Contradiction extractor ──────────────────────────────────────────


def create_contradiction_extractor() -> Agent:
    return Agent(
        name="contradiction_extractor",
        model="gemini-flash-latest",
        instruction=f"""You are a TRIZ expert. Extract the core technical contradiction from this problem:

PROBLEM: {{problem}}

A technical contradiction exists when improving one engineering parameter degrades another.
Identify which parameter we want to IMPROVE and which one WORSENS as a side effect.
Both values MUST be integers strictly between 1 and 39 inclusive.

TRIZ engineering parameters:
{_TRIZ_PARAMETERS_TEXT}

Return JSON with improving_param (int 1-39), worsening_param (int 1-39), and justification (string).
""",
        output_schema=ContradictionResult,
        output_key="contradiction",
        disallow_transfer_to_parent=True,
        disallow_transfer_to_peers=True,
    )


# ── Step 2a: TRIZ lookup (pure deterministic code) ───────────────────────────


class TrizLookupAgent(BaseAgent):
    """Deterministic TRIZ matrix lookup. Zero LLM involvement."""

    async def _run_async_impl(
        self, ctx: InvocationContext
    ) -> AsyncGenerator[Event, None]:
        contradiction = ctx.session.state.get("contradiction", {})
        improving = int(contradiction.get("improving_param", 0))
        worsening = int(contradiction.get("worsening_param", 0))

        try:
            principles = lookup_principles(improving, worsening)
        except ValueError as e:
            logger.error("TRIZ lookup failed: %s", e)
            principles = []

        lookup = {
            "improving_param": improving,
            "improving_name": get_parameter_name(improving),
            "worsening_param": worsening,
            "worsening_name": get_parameter_name(worsening),
            "principles": [get_principle_description(p) for p in principles],
            "principles_count": len(principles),
        }
        ctx.session.state["lookup"] = lookup

        summary = (
            f"TRIZ lookup: {lookup['improving_name']} → {lookup['worsening_name']}, "
            f"{len(principles)} principles found: {[p['name'] for p in lookup['principles']]}"
        )
        logger.info(summary)
        yield Event(
            author=self.name,
            content=genai_types.Content(parts=[genai_types.Part(text=summary)]),
        )


# ── Step 1a: Criteria extractor ───────────────────────────────────────────────


def create_criteria_extractor() -> Agent:
    return Agent(
        name="criteria_extractor",
        model="gemini-flash-latest",
        instruction="""You are a solution evaluator. Extract evaluation criteria from the problem.

PROBLEM: {problem}

Generic criteria (always include all three):
1. "Resolves the technical contradiction"
2. "Technical feasibility"
3. "Cost and scalability"

Problem-specific criteria: derive 2-4 constraints explicitly stated or strongly implied by the problem.
Example: if problem says "keep transport as-is", add "Transport continuity preserved".

Return JSON with generic_criteria (list of 3 strings) and problem_specific_criteria (list of 2-4 strings).""",
        output_schema=EvaluationCriteria,
        output_key="criteria",
        disallow_transfer_to_parent=True,
        disallow_transfer_to_peers=True,
    )


# ── Step 3a: TRIZ candidate generator ────────────────────────────────────────


def create_triz_generator() -> Agent:
    return Agent(
        name="triz_generator",
        model="gemini-flash-latest",
        instruction="""You are a TRIZ solution inventor. Generate solution candidates using TRIZ inventive principles.

PROBLEM: {problem}
CONTRADICTION: Improving "{lookup[improving_name]}" while "{lookup[worsening_name]}" worsens.
TRIZ PRINCIPLES TO APPLY: {lookup[principles]}

For EACH principle listed above, generate ONE concrete, specific solution idea.
Each idea must:
- Directly address the core problem
- Be traceable to its principle (set trace_id to "triz-<principle_number>")
- Be feasible (not science fiction)

Return JSON with a "candidates" list. Each candidate: principle_number (int), principle_name (str), idea (str), trace_id (str).""",
        output_schema=TrizCandidates,
        output_key="triz_candidates",
        disallow_transfer_to_parent=True,
        disallow_transfer_to_peers=True,
    )


# ── Step 3b: SCAMPER candidate generator ─────────────────────────────────────
# Code defines the operators (deterministic); LLM generates content only.


def create_scamper_generator() -> Agent:
    operators_text = "\n".join(
        f"- {op} ({name}): {hint}" for op, name, hint in _SCAMPER_OPERATORS
    )
    return Agent(
        name="scamper_generator",
        model="gemini-flash-latest",
        instruction=f"""You are a creative problem solver using the SCAMPER method.

PROBLEM: {{problem}}

Apply each of the 7 SCAMPER operators below to generate solution ideas.
You MUST generate at least one candidate for S, C, A, M, P, E, R (select the most promising ones).
Return at least 3 candidates total.

SCAMPER OPERATORS (defined by the system — do not change or omit):
{operators_text}

For each candidate: operator (single letter), operator_name (str), idea (str), trace_id = "scamper-<letter>".
Return JSON with a "candidates" list.""",
        output_schema=ScamperCandidates,
        output_key="scamper_candidates",
        disallow_transfer_to_parent=True,
        disallow_transfer_to_peers=True,
    )


# ── Step 4: Evaluator ─────────────────────────────────────────────────────────


def create_evaluator() -> Agent:
    return Agent(
        name="evaluator",
        model="gemini-flash-latest",
        instruction="""You are an objective solution evaluator. Score each candidate solution.

PROBLEM: {problem}

EVALUATION CRITERIA: {criteria}

ALL TRIZ CANDIDATES: {triz_candidates}
ALL SCAMPER CANDIDATES: {scamper_candidates}

For EVERY candidate (TRIZ + SCAMPER), score each criterion 0-100 and sum to total.
Use trace_id to identify candidates (e.g. "triz-35", "scamper-S").

Return JSON with an "evaluations" list. Each item: trace_id (str), scores (dict: criterion→score int), justification (str), total (int = sum of all scores).""",
        output_schema=EvaluationResult,
        output_key="evaluation",
        disallow_transfer_to_parent=True,
        disallow_transfer_to_peers=True,
    )


# ── Step 5: Choice selector (pure deterministic code) ────────────────────────


class ChoiceAgent(BaseAgent):
    """Selects winner by argmax of total scores. Zero LLM involvement."""

    async def _run_async_impl(
        self, ctx: InvocationContext
    ) -> AsyncGenerator[Event, None]:
        evaluation = ctx.session.state.get("evaluation", {})
        evaluations = evaluation.get("evaluations", [])

        if not evaluations:
            choice = {
                "winner_id": "none",
                "winner_total": 0,
                "reasoning": "No evaluations available",
            }
        else:
            winner = max(evaluations, key=lambda e: int(e.get("total", 0)))
            choice = {
                "winner_id": winner["trace_id"],
                "winner_total": winner["total"],
                "reasoning": f"Highest aggregate score ({winner['total']}) across all criteria. argmax selection.",
            }

        ctx.session.state["choice"] = choice
        summary = f"Choice: {choice['winner_id']} (score: {choice.get('winner_total')})"
        logger.info(summary)
        yield Event(
            author=self.name,
            content=genai_types.Content(parts=[genai_types.Part(text=summary)]),
        )


# ── Trail assembler ───────────────────────────────────────────────────────────


class TrailAssemblerAgent(BaseAgent):
    """Assembles the full reasoning trail JSON from all pipeline state keys."""

    async def _run_async_impl(
        self, ctx: InvocationContext
    ) -> AsyncGenerator[Event, None]:
        s = ctx.session.state
        trail = {
            "step1_problem": s.get("problem"),
            "step2_contradiction": s.get("contradiction"),
            "step2a_lookup": s.get("lookup"),
            "step1a_criteria": s.get("criteria"),
            "step3a_triz_candidates": s.get("triz_candidates"),
            "step3b_scamper_candidates": s.get("scamper_candidates"),
            "step4_evaluation": s.get("evaluation"),
            "step5_choice": s.get("choice"),
        }
        ctx.session.state["trail"] = trail

        yield Event(
            author=self.name,
            content=genai_types.Content(
                parts=[
                    genai_types.Part(
                        text=json.dumps(trail, indent=2, ensure_ascii=False)
                    )
                ]
            ),
        )


# ── Root pipeline ─────────────────────────────────────────────────────────────

root_agent = SequentialAgent(
    name="inventive_problem_solver",
    description="Solves any inventive problem using TRIZ contradiction matrix and SCAMPER. Returns a full 5-step reasoning trail.",
    sub_agents=[
        create_problem_normalizer(),
        create_contradiction_extractor(),
        TrizLookupAgent(name="triz_lookup"),
        create_criteria_extractor(),
        ParallelAgent(
            name="candidate_generators",
            sub_agents=[
                create_triz_generator(),
                create_scamper_generator(),
            ],
        ),
        create_evaluator(),
        ChoiceAgent(name="choice_selector"),
        TrailAssemblerAgent(name="trail_assembler"),
    ],
)

app = App(
    root_agent=root_agent,
    name="app",
)
