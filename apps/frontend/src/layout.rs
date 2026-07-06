//! Sheet layout shell (Angular: HeurekaLayout) — masthead, nav, a11y
//! controls, routed content, titleblock footer.

use dioxus::prelude::*;

use crate::Route;
use crate::state::{DARK_MODE, FONT_STEP_INDEX, FONT_STEPS, HIGH_CONTRAST, apply_a11y_effects};

struct SheetMeta {
    ark: &'static str,
    label: &'static str,
    arkusz: &'static str,
}

fn sheet_meta(route: &Route) -> SheetMeta {
    match route {
        Route::MethodsPage {} => SheetMeta {
            ark: "ARK. 2/3",
            label: "KARTA METODOLOGII",
            arkusz: "Karta metodologii",
        },
        Route::TeamPage {} => SheetMeta {
            ark: "ARK. 4/4",
            label: "KARTA PERSONELU",
            arkusz: "Karta personelu",
        },
        Route::ArchitecturePage {} => SheetMeta {
            ark: "ARK. 3/4",
            label: "MAPA ARCHITEKTURY",
            arkusz: "Architektura",
        },
        Route::AnalysisPage {} => SheetMeta {
            ark: "ARK. 1/4",
            label: "TRIZ · SCAMPER",
            arkusz: "Analiza",
        },
    }
}

#[component]
fn NavLink(to: Route, label: &'static str) -> Element {
    let nav = use_navigator();
    let current = use_route::<Route>();
    let active = current == to;
    let href = to.to_string();
    rsx! {
        a {
            href: "{href}",
            "aria-current": if active { "page" },
            onclick: move |e| {
                e.prevent_default();
                nav.push(to.clone());
            },
            "{label}"
        }
    }
}

/// Today's date as `YYYY-MM-DD` (UTC). js-sys imports panic outside wasm, so
/// the server render computes the civil date from `SystemTime` instead; both
/// paths use UTC, keeping SSR and hydration output identical.
#[cfg(target_arch = "wasm32")]
fn today_iso() -> String {
    js_sys::Date::new_0()
        .to_iso_string()
        .as_string()
        .unwrap_or_default()
        .chars()
        .take(10)
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn today_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Days-since-epoch → civil date (Howard Hinnant's algorithm).
    let days = (secs / 86_400) as i64;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

#[component]
pub fn HeurekaLayout() -> Element {
    apply_a11y_effects();

    let route = use_route::<Route>();
    let meta = sheet_meta(&route);
    let today = today_iso();

    let dark = *DARK_MODE.read();
    let contrast = *HIGH_CONTRAST.read();
    let font_index = *FONT_STEP_INDEX.read();
    let can_shrink = font_index > 0;
    let can_enlarge = font_index < FONT_STEPS.len() - 1;

    rsx! {
        div { class: "heureka",
            a { class: "skip", href: "#tresc", "Przejdź do treści" }

            div { class: "sheet",
                div { class: "sheet__inner",
                    header { class: "masthead",
                        p { class: "masthead__brand",
                            "Heureka"
                            span { "." }
                        }
                        nav { class: "nav", aria_label: "Główna nawigacja",
                            NavLink { to: Route::AnalysisPage {}, label: "Analiza" }
                            NavLink { to: Route::MethodsPage {}, label: "Metody" }
                            NavLink { to: Route::ArchitecturePage {}, label: "Architektura" }
                            NavLink { to: Route::TeamPage {}, label: "Zespół" }
                        }
                        div { class: "a11y", role: "group", aria_label: "Ustawienia dostępności",
                            button {
                                r#type: "button",
                                class: if dark { "a11y__btn a11y__btn--on" } else { "a11y__btn" },
                                aria_pressed: "{dark}",
                                onclick: move |_| {
                                    let next = !*DARK_MODE.peek();
                                    *DARK_MODE.write() = next;
                                },
                                span { aria_hidden: "true", "☾" }
                                " Ciemny"
                            }
                            button {
                                r#type: "button",
                                class: if contrast { "a11y__btn a11y__btn--on" } else { "a11y__btn" },
                                aria_pressed: "{contrast}",
                                onclick: move |_| {
                                    let next = !*HIGH_CONTRAST.peek();
                                    *HIGH_CONTRAST.write() = next;
                                },
                                span { aria_hidden: "true", "◐" }
                                " Kontrast"
                            }
                            div { class: "a11y__group", role: "group", aria_label: "Rozmiar tekstu",
                                button {
                                    r#type: "button",
                                    class: "a11y__btn a11y__btn--icon",
                                    disabled: !can_shrink,
                                    aria_label: "Zmniejsz tekst",
                                    onclick: move |_| {
                                        let index = *FONT_STEP_INDEX.peek();
                                        *FONT_STEP_INDEX.write() = index.saturating_sub(1);
                                    },
                                    "A"
                                    span { aria_hidden: "true", "−" }
                                }
                                span { class: "a11y__level", aria_live: "polite",
                                    "{font_index + 1}/{FONT_STEPS.len()}"
                                }
                                button {
                                    r#type: "button",
                                    class: "a11y__btn a11y__btn--icon",
                                    disabled: !can_enlarge,
                                    aria_label: "Powiększ tekst",
                                    onclick: move |_| {
                                        let index = *FONT_STEP_INDEX.peek();
                                        *FONT_STEP_INDEX.write() = (index + 1).min(FONT_STEPS.len() - 1);
                                    },
                                    "A"
                                    span { aria_hidden: "true", "+" }
                                }
                            }
                        }
                        p { class: "masthead__meta",
                            "{meta.ark} · REW. S+"
                            br {}
                            "{meta.label}"
                        }
                    }

                    main { id: "tresc", Outlet::<Route> {} }
                }

                dl { class: "titleblock",
                    div {
                        dt { "Projekt" }
                        dd { "Heureka — inventive problem solving" }
                    }
                    div {
                        dt { "Arkusz" }
                        dd { "{meta.arkusz}" }
                    }
                    div {
                        dt { "Rewizja" }
                        dd { "S+" }
                    }
                    div {
                        dt { "Data" }
                        dd { "{today}" }
                    }
                }
            }
        }
    }
}
