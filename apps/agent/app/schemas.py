from pydantic import BaseModel, Field, field_validator


class ContradictionResult(BaseModel):
    improving_param: int = Field(
        ge=1, le=39, description="TRIZ parameter number to improve (1-39)"
    )
    worsening_param: int = Field(
        ge=1, le=39, description="TRIZ parameter number that worsens (1-39)"
    )
    justification: str = Field(
        description="Why this contradiction captures the core tension"
    )

    @field_validator("improving_param", "worsening_param")
    @classmethod
    def must_be_in_range(cls, v: int) -> int:
        if not 1 <= v <= 39:
            raise ValueError(f"TRIZ parameter must be 1-39, got {v}")
        return v


class TrizCandidate(BaseModel):
    principle_number: int = Field(ge=1, le=40)
    principle_name: str
    idea: str = Field(description="Concrete solution idea applying this principle")
    trace_id: str = Field(description="e.g. triz-35")


class TrizCandidates(BaseModel):
    candidates: list[TrizCandidate] = Field(min_length=1)


class ScamperCandidate(BaseModel):
    operator: str = Field(description="Single letter: S, C, A, M, P, E, or R")
    operator_name: str = Field(description="Full operator name, e.g. Substitute")
    idea: str = Field(
        description="Concrete solution idea applying this SCAMPER operator"
    )
    trace_id: str = Field(description="e.g. scamper-S")


class ScamperCandidates(BaseModel):
    candidates: list[ScamperCandidate] = Field(min_length=3)


class EvaluationCriteria(BaseModel):
    generic_criteria: list[str] = Field(
        description="Universal criteria: resolves contradiction, feasibility, cost/scale"
    )
    problem_specific_criteria: list[str] = Field(
        description="Criteria derived from problem constraints"
    )


class CandidateScore(BaseModel):
    trace_id: str
    scores: dict[str, int] = Field(description="criterion -> score 0-100")
    justification: str
    total: int


class EvaluationResult(BaseModel):
    evaluations: list[CandidateScore]
