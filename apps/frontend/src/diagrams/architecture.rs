//! Architecture map: static SVG/HTML port of the ng-diagram view, with node
//! copy updated for the Rust stack (Dioxus fullstack frontend + agent).

use dioxus::prelude::*;

use super::{port_point, Port};

const W: f64 = 210.0;
const H: f64 = 104.0;
pub const CANVAS_W: f64 = 1350.0 + W + 40.0;
pub const CANVAS_H: f64 = 640.0 + H + 40.0;

struct ArchNode {
    id: &'static str,
    x: f64,
    y: f64,
    eyebrow: &'static str,
    title: &'static str,
    body: &'static str,
    kind: &'static str,
}

const NODES: [ArchNode; 16] = [
    ArchNode {
        id: "user",
        x: 0.0,
        y: 220.0,
        eyebrow: "użytkownik",
        title: "Problem",
        body: "Wpisuje lub dyktuje problem techniczny w interfejsie Heureka.",
        kind: "client",
    },
    ArchNode {
        id: "dioxus",
        x: 270.0,
        y: 110.0,
        eyebrow: "frontend",
        title: "Dioxus",
        body:
            "Rust + WebAssembly: komponenty, sygnały, routing, polski interfejs i kontrolki a11y.",
        kind: "app",
    },
    ArchNode {
        id: "diagram",
        x: 270.0,
        y: 270.0,
        eyebrow: "wizualizacja",
        title: "Diagramy SVG",
        body: "Rysuje pipeline agenta i tę mapę architektury w przeglądarce.",
        kind: "app",
    },
    ArchNode {
        id: "frontendRun",
        x: 540.0,
        y: 30.0,
        eyebrow: "gcp · cloud run",
        title: "Dioxus Fullstack",
        body: "Kontener Axum serwuje WASM, routing aplikacji i funkcje serwerowe.",
        kind: "cloud",
    },
    ArchNode {
        id: "api",
        x: 540.0,
        y: 190.0,
        eyebrow: "server functions",
        title: "Dioxus API",
        body: "/api/solve proxy do agenta oraz /api/speech/transcribe.",
        kind: "api",
    },
    ArchNode {
        id: "speech",
        x: 810.0,
        y: 50.0,
        eyebrow: "google cloud",
        title: "Speech-to-Text",
        body: "Transkrybuje nagrania z mikrofonu po stronie serwera Dioxus.",
        kind: "cloud",
    },
    ArchNode {
        id: "agentService",
        x: 810.0,
        y: 210.0,
        eyebrow: "gcp · cloud run",
        title: "Axum Agent",
        body: "Endpointy /solve i /feedback — serwis agenta w Rust.",
        kind: "agent",
    },
    ArchNode {
        id: "pipeline",
        x: 1080.0,
        y: 210.0,
        eyebrow: "pipeline",
        title: "TRIZ pipeline",
        body: "Normalizacja, kontradykcja, lookup, generatory, ewaluacja i wybór.",
        kind: "agent",
    },
    ArchNode {
        id: "vertex",
        x: 1350.0,
        y: 90.0,
        eyebrow: "vertex ai",
        title: "Gemini",
        body: "LLM odpowiada za kroki wymagające oceny i generowania treści.",
        kind: "cloud",
    },
    ArchNode {
        id: "triz",
        x: 1350.0,
        y: 250.0,
        eyebrow: "deterministyczny kod",
        title: "TRIZ + SCAMPER",
        body: "Macierz 39x39, operatory SCAMPER, argmax wyboru i trail JSON.",
        kind: "data",
    },
    ArchNode {
        id: "github",
        x: 270.0,
        y: 480.0,
        eyebrow: "ci/cd",
        title: "GitHub Actions",
        body: "Cargo fmt, clippy, test, Dioxus build oraz deploy do Cloud Run.",
        kind: "tooling",
    },
    ArchNode {
        id: "nx",
        x: 540.0,
        y: 480.0,
        eyebrow: "monorepo",
        title: "Cargo workspace",
        body: "Dwie aplikacje Rust w workspace: fullstack frontend i agent.",
        kind: "tooling",
    },
    ArchNode {
        id: "docker",
        x: 810.0,
        y: 480.0,
        eyebrow: "artefakty",
        title: "Docker",
        body: "Obrazy dla agenta oraz Dioxus fullstack frontend z binarką server.",
        kind: "tooling",
    },
    ArchNode {
        id: "terraform",
        x: 1080.0,
        y: 480.0,
        eyebrow: "infra as code",
        title: "Terraform",
        body: "Cloud Run, IAM, storage, telemetry outputs i public invoker.",
        kind: "tooling",
    },
    ArchNode {
        id: "deployScripts",
        x: 1350.0,
        y: 480.0,
        eyebrow: "skrypty",
        title: "just deploy",
        body: "Docker build/push, gcloud run deploy i przekazanie AGENT_URL do frontendu.",
        kind: "tooling",
    },
    ArchNode {
        id: "gcp",
        x: 810.0,
        y: 640.0,
        eyebrow: "platforma",
        title: "Google Cloud",
        body: "Cloud Run, Vertex AI, Speech-to-Text, logging i Workload Identity.",
        kind: "cloud",
    },
];

struct ArchEdge {
    source: &'static str,
    target: &'static str,
    source_port: Port,
    target_port: Port,
}

const fn e(
    source: &'static str,
    target: &'static str,
    source_port: Port,
    target_port: Port,
) -> ArchEdge {
    ArchEdge {
        source,
        target,
        source_port,
        target_port,
    }
}

const EDGES: [ArchEdge; 17] = [
    e("user", "dioxus", Port::Right, Port::Left),
    e("user", "diagram", Port::Right, Port::Left),
    e("dioxus", "api", Port::Right, Port::Left),
    e("diagram", "dioxus", Port::Top, Port::Bottom),
    e("frontendRun", "api", Port::Bottom, Port::Top),
    e("api", "speech", Port::Right, Port::Left),
    e("api", "agentService", Port::Right, Port::Left),
    e("agentService", "pipeline", Port::Right, Port::Left),
    e("pipeline", "vertex", Port::Right, Port::Left),
    e("pipeline", "triz", Port::Right, Port::Left),
    e("github", "nx", Port::Right, Port::Left),
    e("nx", "docker", Port::Right, Port::Left),
    e("docker", "gcp", Port::Bottom, Port::Top),
    e("terraform", "gcp", Port::Bottom, Port::Top),
    e("deployScripts", "agentService", Port::Top, Port::Bottom),
    e("gcp", "frontendRun", Port::Top, Port::Bottom),
    e("gcp", "agentService", Port::Top, Port::Bottom),
];

fn node_by_id(id: &str) -> &'static ArchNode {
    NODES
        .iter()
        .find(|n| n.id == id)
        .expect("edge references a known node")
}

#[component]
pub fn ArchitectureDiagram() -> Element {
    rsx! {
        div { class: "diagram-canvas", style: "width: {CANVAS_W}px; height: {CANVAS_H}px;",
            svg {
                width: "{CANVAS_W}",
                height: "{CANVAS_H}",
                view_box: "0 0 {CANVAS_W} {CANVAS_H}",
                defs {
                    marker {
                        id: "architecture-arrow",
                        view_box: "0 0 10 6",
                        ref_x: "9",
                        ref_y: "3",
                        marker_width: "10",
                        marker_height: "6",
                        orient: "auto",
                        path { d: "M0,0 L10,3 L0,6 Z", fill: "var(--ds-color-content-secondary)" }
                    }
                }
                for edge in EDGES.iter() {
                    {
                        let s = node_by_id(edge.source);
                        let t = node_by_id(edge.target);
                        let from = port_point(s.x, s.y, W, H, edge.source_port);
                        let to = port_point(t.x, t.y, W, H, edge.target_port);
                        rsx! {
                            line {
                                class: "edge",
                                x1: "{from.0}",
                                y1: "{from.1}",
                                x2: "{to.0}",
                                y2: "{to.1}",
                                marker_end: "url(#architecture-arrow)",
                            }
                        }
                    }
                }
            }

            for node in NODES.iter() {
                div {
                    id: "{node.id}",
                    class: "diagram-node",
                    style: "left: {node.x}px; top: {node.y}px; width: {W}px; min-height: {H}px;",
                    article { class: "arch-node arch-node--{node.kind}",
                        p { class: "arch-node__eyebrow", "{node.eyebrow}" }
                        h2 { class: "arch-node__title", "{node.title}" }
                        p { class: "arch-node__body", "{node.body}" }
                    }
                }
            }
        }
    }
}
