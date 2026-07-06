//! Architecture page (Angular: ArchitecturePage) — copy updated for the Rust
//! stack; diagram is the hand-rolled SVG port.

use dioxus::prelude::*;

use crate::diagrams::architecture::ArchitectureDiagram;

#[component]
pub fn ArchitecturePage() -> Element {
    rsx! {
        p { class: "eyebrow", "Mapa systemu" }
        h1 { "Architektura" }
        p { class: "lead",
            "Pełny przepływ od użytkownika do agenta: Dioxus fullstack zbiera problem, funkcje serwerowe w tym samym kontenerze pośredniczą w analizie i transkrypcji, a agent w Rust uruchamia pipeline TRIZ/SCAMPER na Google Cloud Platform."
        }

        section { class: "architecture-panel", aria_labelledby: "architecture-h",
            div { class: "architecture-panel__head",
                div {
                    p { class: "architecture-panel__tag", "Widok SVG" }
                    h2 { id: "architecture-h", "Topologia aplikacji" }
                }
                p { class: "architecture-panel__meta",
                    "Dioxus · Axum · Gemini · Vertex AI · Cloud Run · Terraform"
                }
            }
            div { class: "diagram-scroll diagram-scroll--architecture",
                ArchitectureDiagram {}
            }
        }

        section { class: "doc-section", aria_labelledby: "architecture-notes-h",
            h2 { id: "architecture-notes-h", "Warstwy" }
            div { class: "doc-grid",
                article { class: "doc-card",
                    span { class: "doc-card__tag", "UI" }
                    h3 { "Dioxus + WebAssembly" }
                    p {
                        "Interfejs renderuje formularz, trail rozumowania, diagram pipeline i kontrolki dostępności. Wywołania "
                        code { "/api/*" }
                        " obsługują wygenerowane klienty Dioxus server functions."
                    }
                }
                article { class: "doc-card",
                    span { class: "doc-card__tag", "Fullstack" }
                    h3 { "Dioxus server functions" }
                    p {
                        "Ten sam kontener Cloud Run ukrywa integracje serwerowe: przekazuje "
                        code { "/api/solve" }
                        " do agenta i wysyła nagrania audio do Google Cloud Speech-to-Text."
                    }
                }
                article { class: "doc-card",
                    span { class: "doc-card__tag", "Agent" }
                    h3 { "Agent w Rust" }
                    p {
                        "Agent łączy kroki LLM na Vertex AI z deterministycznym lookupem TRIZ, SCAMPER, wyborem przez argmax i złożeniem końcowego traila."
                    }
                }
            }
        }
    }
}
