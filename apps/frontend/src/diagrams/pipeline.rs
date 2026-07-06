//! Pipeline diagram: static SVG/HTML port of the ng-diagram view. Same node
//! data and layout constants as the Angular `PipelineDiagramComponent`; the
//! live highlight states come from the global highlight machine.

use dioxus::prelude::*;

use super::{port_point, Edge, Port};
use crate::state::{highlight_of, NodeHighlight};

const NW: f64 = 180.0; // node width
const NH: f64 = 120.0; // node height
const GAP: f64 = 70.0; // horizontal gap between nodes
const CY: f64 = 200.0; // vertical center
const GW: f64 = 220.0; // group width
const GHEAD: f64 = 64.0; // group header space
const GCHILD_GAP: f64 = 18.0;
const GPAD: f64 = 16.0;
const GH: f64 = GHEAD + NH + GCHILD_GAP + NH + GPAD;
const GY: f64 = CY - GH / 2.0;
const GCHILD1_Y: f64 = GY + GHEAD;
const GCHILD2_Y: f64 = GCHILD1_Y + NH + GCHILD_GAP;

fn sx(slot: f64) -> f64 {
    slot * (NW + GAP)
}

const GX: f64 = 4.0 * (NW + GAP);
pub const CANVAS_W: f64 = GX + GW + GAP + (NW + GAP) * 2.0 + NW + 40.0;
pub const CANVAS_H: f64 = 440.0;

struct AgentNode {
    id: &'static str,
    x: f64,
    y: f64,
    step: &'static str,
    type_label: &'static str,
    deterministic: bool,
    prompt: &'static str,
}

/// Node set mirrors the Rust agent pipeline (`apps/agent/src/pipeline`);
/// prompts shown verbatim, deterministic steps described in Polish.
fn nodes() -> Vec<AgentNode> {
    vec![
        AgentNode {
            id: "problem_normalizer",
            x: sx(0.0),
            y: CY - NH / 2.0,
            step: "krok 1",
            type_label: "LLM",
            deterministic: false,
            prompt: "You are a technical problem analyst. Analyze the inventive problem and output a normalized problem statement that includes:\n- Core challenge (1 sentence)\n- Domain context\n- Key constraints mentioned\n- Desired outcome\n\nBe concise. Do not add information not in the original problem.",
        },
        AgentNode {
            id: "contradiction_extractor",
            x: sx(1.0),
            y: CY - NH / 2.0,
            step: "krok 2",
            type_label: "LLM",
            deterministic: false,
            prompt: "You are a TRIZ expert. Extract the core technical contradiction from this problem:\n\nPROBLEM: {problem}\n\nA technical contradiction exists when improving one engineering parameter degrades another.\nIdentify which parameter we want to IMPROVE and which one WORSENS as a side effect.\nBoth values MUST be integers strictly between 1 and 39 inclusive.\n\nTRIZ engineering parameters: lista 39 parametrów inżynierskich (1-Weight of moving object … 39-Productivity).\n\nReturn JSON with improving_param (int 1-39), worsening_param (int 1-39), and justification (string).",
        },
        AgentNode {
            id: "triz_lookup",
            x: sx(2.0),
            y: CY - NH / 2.0,
            step: "krok 2a · det.",
            type_label: "kod",
            deterministic: true,
            prompt: "Deterministyczny kod Rust — zero LLM.\nBierze improving_param i worsening_param z kroku 2,\nwywołuje lookup_principles() na macierzy TRIZ 39×39,\nzwraca nazwy parametrów i listę zasad wynalazczych.",
        },
        AgentNode {
            id: "criteria_extractor",
            x: sx(3.0),
            y: CY - NH / 2.0,
            step: "krok 2b",
            type_label: "LLM",
            deterministic: false,
            prompt: "You are a solution evaluator. Extract evaluation criteria from the problem.\n\nPROBLEM: {problem}\n\nGeneric criteria (always include all three):\n1. \"Resolves the technical contradiction\"\n2. \"Technical feasibility\"\n3. \"Cost and scalability\"\n\nProblem-specific criteria: derive 2-4 constraints explicitly stated or strongly implied by the problem.\n\nReturn JSON with generic_criteria (list of 3 strings) and problem_specific_criteria (list of 2-4 strings).",
        },
        AgentNode {
            id: "triz_generator",
            x: GX + 20.0,
            y: GCHILD1_Y,
            step: "krok 3a",
            type_label: "LLM",
            deterministic: false,
            prompt: "You are a TRIZ solution inventor. Generate solution candidates using TRIZ inventive principles.\n\nPROBLEM: {problem}\nCONTRADICTION: Improving \"{improving}\" while \"{worsening}\" worsens.\nTRIZ PRINCIPLES TO APPLY: {principles}\n\nFor EACH principle listed above, generate ONE concrete, specific solution idea.\nEach idea must be traceable to its principle (trace_id \"triz-<principle_number>\") and feasible.\n\nReturn JSON with a \"candidates\" list.",
        },
        AgentNode {
            id: "scamper_generator",
            x: GX + 20.0,
            y: GCHILD2_Y,
            step: "krok 3b",
            type_label: "LLM",
            deterministic: false,
            prompt: "You are a creative problem solver using the SCAMPER method.\n\nPROBLEM: {problem}\n\nApply each of the 7 SCAMPER operators (S, C, A, M, P, E, R) to generate solution ideas.\nReturn at least 3 candidates total; trace_id = \"scamper-<letter>\".\n\nReturn JSON with a \"candidates\" list.",
        },
        AgentNode {
            id: "evaluator",
            x: GX + GW + GAP,
            y: CY - NH / 2.0,
            step: "krok 4",
            type_label: "LLM",
            deterministic: false,
            prompt: "You are an objective solution evaluator. Score each candidate solution.\n\nPROBLEM: {problem}\nEVALUATION CRITERIA: {criteria}\nALL TRIZ CANDIDATES: {triz_candidates}\nALL SCAMPER CANDIDATES: {scamper_candidates}\n\nFor EVERY candidate (TRIZ + SCAMPER), score each criterion 0-100 and sum to total.\n\nReturn JSON with an \"evaluations\" list.",
        },
        AgentNode {
            id: "choice_selector",
            x: GX + GW + GAP + NW + GAP,
            y: CY - NH / 2.0,
            step: "krok 5a · det.",
            type_label: "kod",
            deterministic: true,
            prompt: "Deterministyczny kod Rust — zero LLM.\nBierze listę evaluations z kroku 4,\nwybiera kandydata z najwyższym total (argmax),\nzwraca winner_id i winner_total.",
        },
        AgentNode {
            id: "trail_assembler",
            x: GX + GW + GAP + (NW + GAP) * 2.0,
            y: CY - NH / 2.0,
            step: "krok 5b · det.",
            type_label: "kod",
            deterministic: true,
            prompt: "Deterministyczny kod Rust — zero LLM.\nZbiera wyniki wszystkich kroków (problem, contradiction, lookup,\ncriteria, triz_candidates, scamper_candidates, evaluation, choice)\ni składa końcowy obiekt trail zwracany przez API.",
        },
    ]
}

fn edges() -> Vec<Edge> {
    let right = |x: f64| port_point(x, CY - NH / 2.0, NW, NH, Port::Right);
    let left = |x: f64| port_point(x, CY - NH / 2.0, NW, NH, Port::Left);
    vec![
        Edge {
            from: right(sx(0.0)),
            to: left(sx(1.0)),
        },
        Edge {
            from: right(sx(1.0)),
            to: left(sx(2.0)),
        },
        Edge {
            from: right(sx(2.0)),
            to: left(sx(3.0)),
        },
        // criteria → group, group → evaluator (group anchors at its own rect)
        Edge {
            from: right(sx(3.0)),
            to: port_point(GX, GY, GW, GH, Port::Left),
        },
        Edge {
            from: port_point(GX, GY, GW, GH, Port::Right),
            to: left(GX + GW + GAP),
        },
        Edge {
            from: right(GX + GW + GAP),
            to: left(GX + GW + GAP + NW + GAP),
        },
        Edge {
            from: right(GX + GW + GAP + NW + GAP),
            to: left(GX + GW + GAP + (NW + GAP) * 2.0),
        },
    ]
}

fn highlight_class(base: &str, id: &str) -> String {
    match highlight_of(id) {
        NodeHighlight::Active => format!("{base} {base}--active"),
        NodeHighlight::Done => format!("{base} {base}--done"),
        NodeHighlight::Idle => base.to_string(),
    }
}

#[component]
pub fn PipelineDiagram(#[props(default)] id_prefix: String) -> Element {
    rsx! {
        div { class: "diagram-canvas", style: "width: {CANVAS_W}px; height: {CANVAS_H}px;",
            svg {
                width: "{CANVAS_W}",
                height: "{CANVAS_H}",
                view_box: "0 0 {CANVAS_W} {CANVAS_H}",
                defs {
                    marker {
                        id: "pipeline-arrow",
                        view_box: "0 0 10 6",
                        ref_x: "9",
                        ref_y: "3",
                        marker_width: "10",
                        marker_height: "6",
                        orient: "auto",
                        path { d: "M0,0 L10,3 L0,6 Z", fill: "var(--ds-color-content-secondary)" }
                    }
                }
                for edge in edges() {
                    line {
                        class: "edge",
                        x1: "{edge.from.0}",
                        y1: "{edge.from.1}",
                        x2: "{edge.to.0}",
                        y2: "{edge.to.1}",
                        marker_end: "url(#pipeline-arrow)",
                    }
                }
            }

            // ParallelAgent group frame (rendered under its children)
            div {
                id: "{id_prefix}candidate_generators",
                class: "diagram-node",
                style: "left: {GX}px; top: {GY}px; width: {GW}px; height: {GH}px;",
                div { class: highlight_class("par-group", "candidate_generators"),
                    div { class: "par-group__header",
                        span { class: "par-group__step", "krok 3 · równolegle" }
                        span { class: "par-group__type", "równoległy" }
                    }
                    span { class: "par-group__name", "candidate_generators" }
                }
            }

            for node in nodes() {
                div {
                    id: "{id_prefix}{node.id}",
                    class: "diagram-node",
                    style: "left: {node.x}px; top: {node.y}px; width: {NW}px;",
                    div {
                        class: {
                            let base = highlight_class("agent-node", node.id);
                            if node.deterministic { format!("{base} agent-node--baseagent") } else { base }
                        },
                        div { class: "agent-node__header",
                            span { class: "agent-node__step", "{node.step}" }
                            span { class: "agent-node__type", "{node.type_label}" }
                        }
                        span { class: "agent-node__name", "{node.id}" }
                        details { class: "agent-node__prompt",
                            summary {
                                if node.deterministic { "opis logiki" } else { "prompt systemowy" }
                            }
                            pre { class: "agent-node__prompt-text", "{node.prompt}" }
                        }
                    }
                }
            }
        }
    }
}
