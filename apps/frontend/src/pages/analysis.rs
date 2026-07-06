//! Core analysis page (Angular: AnalysisPage): problem form with mic input,
//! phased pipeline animation in a fullscreen modal, and the 5-step trail.

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::api;
use crate::diagrams::pipeline::PipelineDiagram;
use crate::speech::{self, RecordingOutcome};
use crate::state::{
    complete_previous, focus_on, mark_done, reset_highlights, set_active, FOCUS, PROCESSING,
};
use crate::trail::{EvalMode, Trail};

const PHASES: [&str; 4] = [
    "Krok 1/5 — normalizacja problemu…",
    "Krok 2/5 — formułowanie kontradykcji…",
    "Krok 3/5 — generowanie kandydatów…",
    "Krok 4/5 — ewaluacja…",
];
const PHASE_INTERVAL_MS: u32 = 8000;

const PHASE_NODES: [&[&str]; 4] = [
    &["problem_normalizer"],
    &[
        "contradiction_extractor",
        "triz_lookup",
        "criteria_extractor",
    ],
    &[
        "candidate_generators",
        "triz_generator",
        "scamper_generator",
    ],
    &["evaluator"],
];
const COMPLETION_NODES: &[&str] = &["choice_selector", "trail_assembler"];

#[component]
pub fn AnalysisPage() -> Element {
    let mut problem = use_signal(String::new);
    let mut eval_mode = use_signal(|| EvalMode::Rubryka);
    let mut running = use_signal(|| false);
    let mut recording = use_signal(|| false);
    let mut transcribing = use_signal(|| false);
    let mut status = use_signal(String::new);
    let mut speech_status = use_signal(String::new);
    let mut error = use_signal(String::new);
    let mut trail = use_signal(|| Option::<Trail>::None);

    let speech_available = use_resource(speech::speech_input_available);
    let speech_ready = speech_available.read().unwrap_or(false);
    let processing = *PROCESSING.read();

    // Follow the active step inside the modal (replaces ng-diagram's
    // centerOnNode): scroll the modal's copy of the node into view.
    use_effect(move || {
        let focus = *FOCUS.read();
        if let Some((_, node_id)) = focus {
            if *PROCESSING.peek() {
                speech::scroll_node_into_view(&format!("modal-{node_id}"));
            }
        }
    });

    let run = move |e: Event<FormData>| {
        e.prevent_default();
        let text = problem.peek().trim().to_string();
        if text.is_empty() {
            error.set("Opisz problem w polu 01, zanim uruchomisz analizę.".into());
            speech::focus_element("problem");
            return;
        }

        error.set(String::new());
        running.set(true);
        trail.set(None);

        reset_highlights();
        *PROCESSING.write() = true;
        set_active(PHASE_NODES[0]);
        focus_on(PHASE_NODES[0][0]);
        status.set(PHASES[0].into());

        let mode = *eval_mode.peek();
        spawn(async move {
            // Fake phased progress — advances every 8s while the API call runs.
            let ticker = spawn(async move {
                let mut phase = 1;
                loop {
                    TimeoutFuture::new(PHASE_INTERVAL_MS).await;
                    if phase < PHASES.len() {
                        status.set(PHASES[phase].into());
                        let next = PHASE_NODES[phase];
                        complete_previous(next);
                        set_active(next);
                        focus_on(next[0]);
                        phase += 1;
                    }
                }
            });

            let result = api::solve(&text, mode).await;
            ticker.cancel();

            match result {
                Ok(mapped) => {
                    trail.set(Some(mapped));
                    status.set("Analiza zakończona".into());
                    complete_previous(COMPLETION_NODES);
                    set_active(COMPLETION_NODES);
                    focus_on(COMPLETION_NODES[0]);
                    TimeoutFuture::new(1500).await;
                    mark_done(COMPLETION_NODES);
                    *PROCESSING.write() = false;
                    speech::focus_element("wyniki-h");
                }
                Err(_) => {
                    status.set(String::new());
                    error.set(
                        "Nie udało się połączyć z silnikiem analizy. Sprawdź, czy API działa, i spróbuj ponownie."
                            .into(),
                    );
                    reset_highlights();
                    *PROCESSING.write() = false;
                }
            }
            running.set(false);
        });
    };

    let toggle_recording = move |_| {
        if *recording.peek() {
            speech::stop_recording();
            return;
        }
        if *running.peek() || *transcribing.peek() {
            return;
        }

        spawn(async move {
            if !speech::speech_input_available().await {
                error.set("Ta przeglądarka nie obsługuje nagrywania z mikrofonu.".into());
                return;
            }
            error.set(String::new());
            recording.set(true);
            speech_status.set("Nagrywanie z mikrofonu...".into());

            let outcome = speech::record().await;
            recording.set(false);

            match outcome {
                RecordingOutcome::Failed => {
                    speech_status.set(String::new());
                    error.set(
                        "Nie udało się uruchomić mikrofonu. Sprawdź uprawnienia przeglądarki."
                            .into(),
                    );
                }
                RecordingOutcome::Empty => {
                    speech_status.set(String::new());
                    error.set("Nie nagrano dźwięku. Spróbuj jeszcze raz.".into());
                }
                RecordingOutcome::Ok(rec) => {
                    transcribing.set(true);
                    speech_status.set("Transkrypcja nagrania...".into());
                    match api::transcribe(&rec.audio_base64, &rec.mime_type).await {
                        Ok(transcript) if !transcript.is_empty() => {
                            let current = problem.peek().trim().to_string();
                            problem.set(if current.is_empty() {
                                transcript
                            } else {
                                format!("{current}\n{transcript}")
                            });
                            speech_status.set("Tekst z mikrofonu dodany do zgłoszenia.".into());
                            speech::focus_element("problem");
                        }
                        Ok(_) => {
                            error.set("Nie rozpoznano tekstu w nagraniu.".into());
                        }
                        Err(_) => {
                            error.set(
                                "Nie udało się rozpoznać mowy. Sprawdź konfigurację Google Cloud Speech-to-Text."
                                    .into(),
                            );
                            speech_status.set(String::new());
                        }
                    }
                    transcribing.set(false);
                }
            }
        });
    };

    rsx! {
        section { aria_labelledby: "hero-h",
            p { class: "eyebrow", "System rozwiązywania problemów wynalazczych" }
            h1 { id: "hero-h", "Od problemu do wynalazku" }
            p { class: "lead",
                "Wpisz dowolny problem techniczny i wybierz sposób oceny kandydatów. Heureka sformułuje kontradykcję, wygeneruje pomysły metodami TRIZ i SCAMPER, oceni każdy z nich i uzasadni wybór — krok po kroku, do wglądu."
            }

            form { class: "form", novalidate: true, onsubmit: run,
                div { class: "form__head",
                    label { r#for: "problem", "Zgłoszenie problemu" }
                    div { class: "form__head-actions",
                        button {
                            class: if recording() { "mic-btn mic-btn--recording" } else { "mic-btn" },
                            r#type: "button",
                            aria_pressed: "{recording()}",
                            disabled: running() || transcribing() || !speech_ready,
                            onclick: toggle_recording,
                            span { class: "material-icons", aria_hidden: "true",
                                if recording() {
                                    "stop"
                                } else {
                                    "mic"
                                }
                            }
                            span {
                                if recording() {
                                    "Zatrzymaj"
                                } else if transcribing() {
                                    "Transkrypcja"
                                } else {
                                    "Mikrofon"
                                }
                            }
                        }
                        span { aria_hidden: "true", "Pole 01" }
                    }
                }
                textarea {
                    id: "problem",
                    name: "problem",
                    aria_describedby: "form-error speech-status problem-hint",
                    placeholder: "Opisz problem: co ma działać lepiej, w jakich warunkach występuje trudność i czego nie wolno zmienić…",
                    value: "{problem}",
                    oninput: move |e| problem.set(e.value()),
                }
                if !speech_status().is_empty() {
                    p {
                        class: "form__speech-status",
                        id: "speech-status",
                        role: "status",
                        "{speech_status}"
                    }
                }

                div { class: "form__options",
                    div { class: "opt",
                        p { class: "opt__legend", "Metody generowania rozwiązań" }
                        div { class: "chips",
                            span { class: "chip chip--fixed",
                                "TRIZ "
                                span { class: "chip__req", "matryca" }
                            }
                            span { class: "chip chip--fixed",
                                "SCAMPER "
                                span { class: "chip__req", "operatory" }
                            }
                        }
                    }
                    fieldset { class: "opt",
                        legend { "Sposób ewaluacji kandydatów" }
                        div { class: "chips",
                            for (mode , value , label) in [
                                (EvalMode::Rubryka, "rubryka", "Rubryka ważona"),
                                (EvalMode::Pugh, "pugh", "Macierz Pugha"),
                                (EvalMode::Pary, "pary", "Porównanie parami"),
                            ]
                            {
                                label { class: "chip",
                                    input {
                                        r#type: "radio",
                                        name: "eval",
                                        value: "{value}",
                                        checked: eval_mode() == mode,
                                        onchange: move |_| eval_mode.set(mode),
                                    }
                                    " {label}"
                                }
                            }
                        }
                    }
                }

                if !error().is_empty() {
                    p {
                        class: "form__error",
                        id: "form-error",
                        role: "alert",
                        "{error}"
                    }
                }

                div { class: "form__foot",
                    span { class: "form__hint", id: "problem-hint",
                        "Dowolny problem wynalazczy — im konkretniej, tym lepiej"
                    }
                    button { class: "btn", r#type: "submit", disabled: running(),
                        if running() {
                            span { class: "btn__spin", aria_hidden: "true" }
                            " Analizuję…"
                        } else {
                            "Uruchom analizę"
                        }
                    }
                }
            }
        }

        section { class: "pipeline-section", aria_labelledby: "pipeline-h",
            details { class: "pipeline__drawer", open: true,
                summary { class: "pipeline__trigger",
                    span { id: "pipeline-h", "Architektura pipeline" }
                    span { class: "pipeline__meta", "Pipeline · 8 węzłów · 1 gałąź równoległa" }
                }
                div { class: "pipeline__diagram-wrap",
                    div { class: "diagram-scroll diagram-scroll--pipeline",
                        PipelineDiagram { id_prefix: "" }
                    }
                }
            }
        }

        if processing {
            div {
                class: "pipeline-modal",
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "pipeline-modal-h",
                div { class: "pipeline-modal__panel",
                    div { class: "pipeline-modal__head",
                        span {
                            id: "pipeline-modal-h",
                            class: "pipeline-modal__title",
                            "Pipeline w trakcie"
                        }
                        span { class: "pipeline-modal__status", role: "status", "{status}" }
                    }
                    div { class: "pipeline-modal__body",
                        div { class: "diagram-scroll diagram-scroll--pipeline",
                            PipelineDiagram { id_prefix: "modal-" }
                        }
                    }
                }
            }
        }

        section { class: "results", aria_labelledby: "wyniki-h",
            div { class: "results__head",
                h2 { id: "wyniki-h", tabindex: "-1", "Ścieżka rozumowania" }
                p { class: "results__status", role: "status", "{status}" }
            }

            if let Some(t) = trail() {
                {trail_view(&t)}
            } else if running() {
                div { class: "loader", aria_hidden: "true",
                    span { class: "loader__spin" }
                    p { class: "loader__label", "{status}" }
                }
            } else {
                p { class: "empty",
                    "Wyniki pojawią się tutaj po uruchomieniu analizy — pięć kroków: problem → kontradykcja → kandydaci → ewaluacja → wybór."
                }
            }
        }
    }
}

fn trail_view(t: &Trail) -> Element {
    let top_score = t
        .ranked
        .first()
        .map(|c| c.score)
        .filter(|s| *s != 0)
        .unwrap_or(1);
    let eval_label = t.eval_mode.label();

    rsx! {
        div { class: "trail",
            section { class: "step", aria_labelledby: "s1",
                div { class: "step__no", aria_hidden: "true", "1" }
                p { class: "step__kicker", "Krok 1 · wejście znormalizowane" }
                h3 { id: "s1", "Problem" }
                p { class: "note", "{t.problem}" }
            }

            section { class: "step", aria_labelledby: "s2",
                div { class: "step__no", aria_hidden: "true", "2" }
                p { class: "step__kicker", "Krok 2 · LLM proponuje, kod waliduje zakres 1–39" }
                h3 { id: "s2", "Kontradykcja techniczna" }
                div { class: "contradiction",
                    div { class: "param param--better",
                        span { class: "param__tag", "{t.contradiction.better.tag}" }
                        p { class: "param__name", "{t.contradiction.better.name}" }
                    }
                    span { class: "vs", aria_hidden: "true", "×" }
                    div { class: "param param--worse",
                        span { class: "param__tag", "{t.contradiction.worse.tag}" }
                        p { class: "param__name", "{t.contradiction.worse.name}" }
                    }
                }
                p { class: "matrix",
                    "MATRYCA[{t.contradiction.better.param_number}, {t.contradiction.worse.param_number}] → zasady "
                    b { "{t.contradiction.principles}" }
                    " · lookup deterministyczny"
                }
            }

            section { class: "step", aria_labelledby: "s3",
                div { class: "step__no", aria_hidden: "true", "3" }
                p { class: "step__kicker", "Krok 3 · po jednym kandydacie na zasadę / krok metody" }
                h3 { id: "s3", "Kandydaci ({t.candidate_count})" }
                div { class: "methods",
                    for group in t.groups.iter() {
                        div {
                            h4 { class: "method__title", "Metoda — {group.label}" }
                            for c in group.candidates.iter() {
                                article { class: "cand",
                                    span { class: "cand__src", "{c.source}" }
                                    p { class: "cand__name", "{c.name}" }
                                    p { "{c.description}" }
                                }
                            }
                        }
                    }
                }
            }

            section { class: "step", aria_labelledby: "s4",
                div { class: "step__no", aria_hidden: "true", "4" }
                p { class: "step__kicker", "Krok 4 · metoda: {eval_label}, oceny z uzasadnieniami" }
                h3 { id: "s4", "Ewaluacja" }
                table { class: "scores",
                    thead {
                        tr {
                            th { scope: "col", "Kandydat" }
                            th { scope: "col", "Źródło" }
                            th { scope: "col", "Wynik łączny" }
                        }
                    }
                    tbody {
                        for (i , c) in t.ranked.iter().enumerate() {
                            tr { class: if i == 0 { "win" },
                                td { "{c.name}" }
                                td { "{c.source}" }
                                td {
                                    div { class: "scores__result",
                                        span {
                                            class: "scores__track",
                                            aria_hidden: "true",
                                            span {
                                                class: "bar",
                                                style: "width: {c.score as f64 / top_score as f64 * 100.0}%;",
                                            }
                                        }
                                        span { class: "score", "{c.score}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section { class: "step", aria_labelledby: "s5",
                div { class: "step__no", aria_hidden: "true", "5" }
                p { class: "step__kicker",
                    "Krok 5 · deterministyczny argmax — wybiera kod, nie model"
                }
                h3 { id: "s5", "Wybór" }
                div { class: "choice",
                    p { class: "stamp", aria_hidden: "true", "Wybrano" }
                    p { class: "choice__src", "{t.winner.name} · {t.winner.source}" }
                    h4 { "Rozwiązanie" }
                    p { class: "choice__idea", "{t.winner.description}" }

                    div { class: "choice__why",
                        h5 { "Dlaczego ten kandydat jest top 1" }
                        ul {
                            li {
                                "Najwyższy wynik łączny: "
                                b { "{t.winner.score}" }
                                " w ocenie metodą „{eval_label}\" — wybór wykonał deterministyczny argmax po wszystkich {t.candidate_count} kandydatach z obu metod."
                            }
                            if let Some(second) = &t.runner_up {
                                li {
                                    "Przewaga "
                                    b { "+{t.winner.score - second.score} pkt" }
                                    " nad drugim w rankingu ({second.name} · {second.score} pkt)."
                                }
                            }
                        }
                        if !t.winner.breakdown.is_empty() {
                            p { class: "choice__breakdown-title", "Oceny per kryterium:" }
                            dl { class: "choice__breakdown",
                                for item in t.winner.breakdown.iter() {
                                    div {
                                        dt { "{item.criterion}" }
                                        dd { "{item.score}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
