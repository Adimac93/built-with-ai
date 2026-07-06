//! Global app state: runtime config, accessibility preferences, and the
//! pipeline-diagram highlight machine. GlobalSignals replace Angular's
//! root-provided signal services.

use std::collections::HashMap;

use dioxus::prelude::*;

// ── Runtime config (Angular: ConfigProvider + APP_INITIALIZER) ──

/// Historical startup hook from the static frontend. The fullstack build uses
/// Dioxus server functions, so there is no runtime API base URL to load.
pub async fn load_config() {}

// ── Accessibility (Angular: AccessibilityService) ──

/// Discrete text-scale steps, expressed as a percentage of the browser default.
pub const FONT_STEPS: [u32; 4] = [100, 115, 130, 150];
const STORAGE_CONTRAST: &str = "heureka.a11y.contrast";
const STORAGE_DARK: &str = "heureka.a11y.dark";
const STORAGE_FONT_STEP: &str = "heureka.a11y.fontStep";

pub static HIGH_CONTRAST: GlobalSignal<bool> =
    Signal::global(|| storage_get(STORAGE_CONTRAST) == Some("true".into()));
pub static DARK_MODE: GlobalSignal<bool> =
    Signal::global(|| storage_get(STORAGE_DARK) == Some("true".into()));
pub static FONT_STEP_INDEX: GlobalSignal<usize> = Signal::global(|| {
    storage_get(STORAGE_FONT_STEP)
        .and_then(|raw| raw.parse::<usize>().ok())
        .filter(|i| *i < FONT_STEPS.len())
        .unwrap_or(0)
});

#[cfg(target_arch = "wasm32")]
fn storage_get(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()??
        .get_item(key)
        .ok()?
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_get(_key: &str) -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
fn storage_set(key: &str, value: &str) {
    // Storage unavailable (private mode) — preference stays in-memory.
    if let Some(Ok(Some(storage))) = web_sys::window().map(|w| w.local_storage()) {
        let _ = storage.set_item(key, value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_set(_key: &str, _value: &str) {}

#[cfg(target_arch = "wasm32")]
fn set_root_class(class_name: &str, on: bool) {
    use wasm_bindgen::JsCast;
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = root.class_list().toggle_with_force(class_name, on);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn set_root_class(_class_name: &str, _on: bool) {}

#[cfg(target_arch = "wasm32")]
fn set_root_font_size(percent: u32) {
    use wasm_bindgen::JsCast;
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = root
            .style()
            .set_property("font-size", &format!("{percent}%"));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn set_root_font_size(_percent: u32) {}

/// Reflects the a11y signals onto `<html>` (classes + font-size) and
/// persists them — the CSS token overrides react to the classes.
pub fn apply_a11y_effects() {
    use_effect(|| {
        let on = *HIGH_CONTRAST.read();
        set_root_class("a11y-contrast", on);
        storage_set(STORAGE_CONTRAST, if on { "true" } else { "false" });
    });
    use_effect(|| {
        let on = *DARK_MODE.read();
        set_root_class("a11y-dark", on);
        storage_set(STORAGE_DARK, if on { "true" } else { "false" });
    });
    use_effect(|| {
        let index = *FONT_STEP_INDEX.read();
        set_root_font_size(FONT_STEPS[index]);
        storage_set(STORAGE_FONT_STEP, &index.to_string());
    });
}

// ── Diagram highlight machine (Angular: DiagramHighlightService) ──

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeHighlight {
    Idle,
    Active,
    Done,
}

pub static HIGHLIGHTS: GlobalSignal<HashMap<&'static str, NodeHighlight>> =
    Signal::global(HashMap::new);
/// True while the fullscreen pipeline modal is open.
pub static PROCESSING: GlobalSignal<bool> = Signal::global(|| false);
/// Node the modal viewport should scroll to; the token makes re-focusing the
/// same node re-fire the scroll effect.
pub static FOCUS: GlobalSignal<Option<(u64, &'static str)>> = Signal::global(|| None);

pub fn highlight_of(id: &str) -> NodeHighlight {
    HIGHLIGHTS
        .read()
        .get(id)
        .copied()
        .unwrap_or(NodeHighlight::Idle)
}

pub fn set_active(ids: &[&'static str]) {
    let mut map = HIGHLIGHTS.write();
    for id in ids {
        map.insert(id, NodeHighlight::Active);
    }
}

/// Marks every currently-active node done, except the ones about to activate.
pub fn complete_previous(next: &[&'static str]) {
    let mut map = HIGHLIGHTS.write();
    let keys: Vec<&'static str> = map
        .iter()
        .filter(|(id, state)| **state == NodeHighlight::Active && !next.contains(*id))
        .map(|(id, _)| *id)
        .collect();
    for id in keys {
        map.insert(id, NodeHighlight::Done);
    }
}

pub fn mark_done(ids: &[&'static str]) {
    let mut map = HIGHLIGHTS.write();
    for id in ids {
        map.insert(id, NodeHighlight::Done);
    }
}

pub fn focus_on(id: &'static str) {
    let token = (*FOCUS.read()).map(|(t, _)| t + 1).unwrap_or(0);
    *FOCUS.write() = Some((token, id));
}

pub fn reset_highlights() {
    HIGHLIGHTS.write().clear();
    *FOCUS.write() = None;
}
