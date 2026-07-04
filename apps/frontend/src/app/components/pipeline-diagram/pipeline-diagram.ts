import { Component } from '@angular/core';
import {
  NgDiagramBackgroundComponent,
  NgDiagramComponent,
  NgDiagramMarkerComponent,
  NgDiagramNodeTemplateMap,
  initializeModel,
  provideNgDiagram,
} from 'ng-diagram';
import { AgentNodeComponent } from './agent-node/agent-node';
import { ParallelGroupComponent } from './parallel-group/parallel-group';

// Layout constants
const NW = 180;  // node width
const NH = 120;  // node height
const GAP = 70;  // horizontal gap between nodes
const CY = 200;  // vertical center
const GW = 220;  // group width
const GH = 310;  // group height
const GY = CY - GH / 2;  // group top y

// x position for sequential slot (0-based)
function sx(slot: number): number {
  return slot * (NW + GAP);
}

// Group x starts after 4 sequential nodes
const GX = sx(4);

@Component({
  selector: 'app-pipeline-diagram',
  imports: [NgDiagramComponent, NgDiagramBackgroundComponent, NgDiagramMarkerComponent],
  providers: [provideNgDiagram()],
  template: `
    <ng-diagram [model]="model" [nodeTemplateMap]="nodeTemplateMap">
      <ng-diagram-background />
      <ng-diagram-marker>
        <svg>
          <defs>
            <marker
              id="pipeline-arrow"
              viewBox="0 0 10 6"
              refX="9"
              refY="3"
              markerWidth="10"
              markerHeight="6"
              orient="auto"
            >
              <path d="M0,0 L10,3 L0,6 Z" fill="context-stroke" />
            </marker>
          </defs>
        </svg>
      </ng-diagram-marker>
    </ng-diagram>
  `,
  styles: `
    :host {
      display: flex;
      height: 500px;
    }
  `,
})
export class PipelineDiagramComponent {
  readonly nodeTemplateMap = new NgDiagramNodeTemplateMap([
    ['agent', AgentNodeComponent],
    ['parallel-group', ParallelGroupComponent],
  ]);

  readonly model = initializeModel({
    nodes: [
      {
        id: 'problem_normalizer',
        type: 'agent',
        position: { x: sx(0), y: CY - NH / 2 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'problem_normalizer',
          agentType: 'LlmAgent',
          step: 'krok 1',
          isDeterministic: false,
          prompt: `You are a technical problem analyst. Analyze the inventive problem and output a normalized problem statement that includes:
- Core challenge (1 sentence)
- Domain context
- Key constraints mentioned
- Desired outcome

Be concise. Do not add information not in the original problem.`,
        },
      },
      {
        id: 'contradiction_extractor',
        type: 'agent',
        position: { x: sx(1), y: CY - NH / 2 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'contradiction_extractor',
          agentType: 'LlmAgent',
          step: 'krok 2',
          isDeterministic: false,
          prompt: `You are a TRIZ expert. Extract the core technical contradiction from this problem:

PROBLEM: {problem}

A technical contradiction exists when improving one engineering parameter degrades another.
Identify which parameter we want to IMPROVE and which one WORSENS as a side effect.
Both values MUST be integers strictly between 1 and 39 inclusive.

TRIZ engineering parameters:
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

Return JSON with improving_param (int 1-39), worsening_param (int 1-39), and justification (string).`,
        },
      },
      {
        id: 'triz_lookup',
        type: 'agent',
        position: { x: sx(2), y: CY - NH / 2 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'triz_lookup',
          agentType: 'BaseAgent',
          step: 'krok 2a · det.',
          isDeterministic: true,
          prompt: `Deterministyczny kod Python — zero LLM.
Odczytuje improving_param i worsening_param ze stanu sesji,
wywołuje lookup_principles() na macierzy TRIZ 39×39,
zapisuje nazwy parametrów i listę zasad wynalazczych.`,
        },
      },
      {
        id: 'criteria_extractor',
        type: 'agent',
        position: { x: sx(3), y: CY - NH / 2 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'criteria_extractor',
          agentType: 'LlmAgent',
          step: 'krok 2b',
          isDeterministic: false,
          prompt: `You are a solution evaluator. Extract evaluation criteria from the problem.

PROBLEM: {problem}

Generic criteria (always include all three):
1. "Resolves the technical contradiction"
2. "Technical feasibility"
3. "Cost and scalability"

Problem-specific criteria: derive 2-4 constraints explicitly stated or strongly implied by the problem.
Example: if problem says "keep transport as-is", add "Transport continuity preserved".

Return JSON with generic_criteria (list of 3 strings) and problem_specific_criteria (list of 2-4 strings).`,
        },
      },
      // ParallelAgent group
      {
        id: 'candidate_generators',
        type: 'parallel-group',
        isGroup: true,
        position: { x: GX, y: GY },
        size: { width: GW, height: GH },
        autoSize: false,
        data: {
          name: 'candidate_generators',
          step: 'krok 3 · równolegle',
        },
      },
      // Children inside group (global coordinates)
      {
        id: 'triz_generator',
        type: 'agent',
        groupId: 'candidate_generators',
        position: { x: GX + 20, y: GY + 55 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'triz_generator',
          agentType: 'LlmAgent',
          step: 'krok 3a',
          isDeterministic: false,
          prompt: `You are a TRIZ solution inventor. Generate solution candidates using TRIZ inventive principles.

PROBLEM: {problem}
CONTRADICTION: Improving "{lookup_improving_name}" while "{lookup_worsening_name}" worsens.
TRIZ PRINCIPLES TO APPLY: {lookup_principles_text}

For EACH principle listed above, generate ONE concrete, specific solution idea.
Each idea must:
- Directly address the core problem
- Be traceable to its principle (set trace_id to "triz-<principle_number>")
- Be feasible (not science fiction)

Return JSON with a "candidates" list. Each candidate: principle_number (int), principle_name (str), idea (str), trace_id (str).`,
        },
      },
      {
        id: 'scamper_generator',
        type: 'agent',
        groupId: 'candidate_generators',
        position: { x: GX + 20, y: GY + 55 + NH + 20 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'scamper_generator',
          agentType: 'LlmAgent',
          step: 'krok 3b',
          isDeterministic: false,
          prompt: `You are a creative problem solver using the SCAMPER method.

PROBLEM: {problem}

Apply each of the 7 SCAMPER operators below to generate solution ideas.
You MUST generate at least one candidate for S, C, A, M, P, E, R (select the most promising ones).
Return at least 3 candidates total.

SCAMPER OPERATORS (defined by the system — do not change or omit):
- S (Substitute): What materials, processes, or components can be substituted?
- C (Combine): What elements or ideas can be merged or combined?
- A (Adapt): What can be adapted or borrowed from other domains?
- M (Modify/Magnify/Minimize): What can be modified, magnified, minimized, or rearranged?
- P (Put to other uses): How can existing elements be used for different purposes?
- E (Eliminate): What can be removed, simplified, or reduced to its core?
- R (Reverse/Rearrange): What can be reversed, inverted, or reordered?

For each candidate: operator (single letter), operator_name (str), idea (str), trace_id = "scamper-<letter>".
Return JSON with a "candidates" list.`,
        },
      },
      // Sequential nodes continued
      {
        id: 'evaluator',
        type: 'agent',
        position: { x: GX + GW + GAP, y: CY - NH / 2 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'evaluator',
          agentType: 'LlmAgent',
          step: 'krok 4',
          isDeterministic: false,
          prompt: `You are an objective solution evaluator. Score each candidate solution.

PROBLEM: {problem}

EVALUATION CRITERIA: {criteria}

ALL TRIZ CANDIDATES: {triz_candidates}
ALL SCAMPER CANDIDATES: {scamper_candidates}

For EVERY candidate (TRIZ + SCAMPER), score each criterion 0-100 and sum to total.
Use trace_id to identify candidates (e.g. "triz-35", "scamper-S").

Return JSON with an "evaluations" list. Each item: trace_id (str), scores (dict: criterion→score int), justification (str), total (int = sum of all scores).`,
        },
      },
      {
        id: 'choice_selector',
        type: 'agent',
        position: { x: GX + GW + GAP + NW + GAP, y: CY - NH / 2 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'choice_selector',
          agentType: 'BaseAgent',
          step: 'krok 5a · det.',
          isDeterministic: true,
          prompt: `Deterministyczny kod Python — zero LLM.
Pobiera listę evaluations ze stanu sesji,
wybiera kandydata z najwyższym total (argmax),
zapisuje winner_id i winner_total.`,
        },
      },
      {
        id: 'trail_assembler',
        type: 'agent',
        position: { x: GX + GW + GAP + (NW + GAP) * 2, y: CY - NH / 2 },
        size: { width: NW, height: NH },
        autoSize: true,
        data: {
          name: 'trail_assembler',
          agentType: 'BaseAgent',
          step: 'krok 5b · det.',
          isDeterministic: true,
          prompt: `Deterministyczny kod Python — zero LLM.
Zbiera wszystkie klucze stanu sesji (problem, contradiction, lookup,
criteria, triz_candidates, scamper_candidates, evaluation, choice)
i składa końcowy obiekt trail zwracany przez API.`,
        },
      },
    ],
    edges: [
      { id: 'e1', source: 'problem_normalizer', sourcePort: 'port-right', target: 'contradiction_extractor', targetPort: 'port-left', targetArrowhead: 'pipeline-arrow', data: {} },
      { id: 'e2', source: 'contradiction_extractor', sourcePort: 'port-right', target: 'triz_lookup', targetPort: 'port-left', targetArrowhead: 'pipeline-arrow', data: {} },
      { id: 'e3', source: 'triz_lookup', sourcePort: 'port-right', target: 'criteria_extractor', targetPort: 'port-left', targetArrowhead: 'pipeline-arrow', data: {} },
      { id: 'e4', source: 'criteria_extractor', sourcePort: 'port-right', target: 'candidate_generators', targetPort: 'port-left', targetArrowhead: 'pipeline-arrow', data: {} },
      { id: 'e5', source: 'candidate_generators', sourcePort: 'port-right', target: 'evaluator', targetPort: 'port-left', targetArrowhead: 'pipeline-arrow', data: {} },
      { id: 'e6', source: 'evaluator', sourcePort: 'port-right', target: 'choice_selector', targetPort: 'port-left', targetArrowhead: 'pipeline-arrow', data: {} },
      { id: 'e7', source: 'choice_selector', sourcePort: 'port-right', target: 'trail_assembler', targetPort: 'port-left', targetArrowhead: 'pipeline-arrow', data: {} },
    ],
  });
}
