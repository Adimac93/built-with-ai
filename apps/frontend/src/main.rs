//! Heureka frontend — Dioxus (Rust + WebAssembly) port of the Angular app.

mod api;
mod diagrams;
mod layout;
mod pages;
mod speech;
mod state;
mod trail;

use dioxus::document;
use dioxus::prelude::*;

use layout::HeurekaLayout;
use pages::analysis::AnalysisPage;
use pages::architecture::ArchitecturePage;
use pages::methods::MethodsPage;
use pages::team::TeamPage;

// Variants intentionally share the `Page` postfix — they name page components.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[layout(HeurekaLayout)]
    #[route("/")]
    AnalysisPage {},
    #[route("/metody")]
    MethodsPage {},
    #[route("/architektura")]
    ArchitecturePage {},
    #[route("/zespol")]
    TeamPage {},
}

#[component]
fn App() -> Element {
    // Runtime config (Angular's APP_INITIALIZER equivalent) — non-blocking.
    use_future(state::load_config);
    use_effect(|| {
        document::eval("document.documentElement.lang = 'pl';");
    });

    rsx! {
        document::Link { rel: "icon", r#type: "image/x-icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap",
        }
        document::Stylesheet { href: "https://fonts.googleapis.com/icon?family=Material+Icons" }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Big+Shoulders+Display:wght@600;800&family=IBM+Plex+Sans:ital,wght@0,400;0,500;0,600;1,400&family=IBM+Plex+Mono:ital,wght@0,400;0,500;1,400&display=swap",
        }
        document::Stylesheet { href: asset!("/assets/heureka.css") }
        document::Stylesheet { href: asset!("/assets/diagram.css") }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
