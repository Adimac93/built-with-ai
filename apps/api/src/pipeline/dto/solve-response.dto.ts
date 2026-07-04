import { ApiProperty } from '@nestjs/swagger';

export class SolveResponseDto {
  @ApiProperty({ description: 'Normalized problem statement' })
  step1_problem!: string;

  @ApiProperty({
    description: 'TRIZ technical contradiction (improving/worsening params 1-39)',
  })
  step2_contradiction!: {
    improving_param: number;
    worsening_param: number;
    justification: string;
  };

  @ApiProperty({ description: 'TRIZ matrix lookup result with inventive principles' })
  step2a_lookup!: {
    improving_param: number;
    worsening_param: number;
    improving_name: string;
    worsening_name: string;
    principles: number[];
    principle_names: string[];
  };

  @ApiProperty({ description: 'Evaluation criteria (generic + problem-specific)' })
  step2b_criteria!: {
    generic_criteria: string[];
    problem_specific_criteria: string[];
  };

  @ApiProperty({ description: 'TRIZ-derived solution candidates with trace IDs' })
  step3a_triz_candidates!: {
    candidates: Array<{
      principle_number: number;
      principle_name: string;
      idea: string;
      trace_id: string;
    }>;
  };

  @ApiProperty({ description: 'SCAMPER-derived solution candidates with trace IDs' })
  step3b_scamper_candidates!: {
    candidates: Array<{
      operator: string;
      operator_name: string;
      idea: string;
      trace_id: string;
    }>;
  };

  @ApiProperty({ description: 'Scored evaluation of all candidates' })
  step4_evaluation!: {
    evaluations: Array<{
      trace_id: string;
      scores: Record<string, number>;
      justification: string;
      total: number;
    }>;
  };

  @ApiProperty({ description: 'Winning candidate chosen by argmax total score' })
  step5_choice!: {
    winner_id: string;
    winner_idea: string;
    score: number;
  };
}
