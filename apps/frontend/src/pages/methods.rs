//! Static methodology docs (Angular: MethodsPage). Content ported verbatim.

use dioxus::prelude::*;

struct DocCard {
    tag: &'static str,
    title: &'static str,
    body: &'static str,
    points: &'static [&'static str],
    ordered: bool,
    how: &'static str,
}

const GENERATION: [DocCard; 2] = [
    DocCard {
        tag: "G-01 · klasyka wynalazczości",
        title: "TRIZ — matryca kontradykcji",
        body: "Metoda Altszullera zbudowana na analizie tysięcy patentów: każdy trudny problem to sprzeczność techniczna — poprawiając jeden parametr, psujesz inny.",
        points: &[
            "Problem → para parametrów (poprawiany × pogarszany) z listy 39.",
            "Lookup w matrycy 39×39 → zasady wynalazcze (1–40).",
            "Jeden kandydat na każdą zwróconą zasadę.",
        ],
        ordered: true,
        how: "lookup w matrycy, walidacja zakresu · LLM: dobór parametrów, treść pomysłu",
    },
    DocCard {
        tag: "G-02 · checklista przekształceń",
        title: "SCAMPER",
        body: "Siedem operatorów, które systematycznie „przekręcają\" istniejące rozwiązanie: Substitute, Combine, Adapt, Modify, Put\u{a0}to\u{a0}another\u{a0}use, Eliminate, Reverse.",
        points: &[
            "Iteracja po operatorach S-C-A-M-P-E-R.",
            "Dla każdego operatora: pytanie przekształcające problem.",
            "Jeden kandydat na operator.",
        ],
        ordered: true,
        how: "pętla operatorów · LLM: odpowiedź na pytanie operatora",
    },
];

const EVALUATION: [DocCard; 3] = [
    DocCard {
        tag: "E-01 · domyślna",
        title: "Rubryka ważona",
        body: "Każdy kandydat dostaje ocenę 0–100 w kilku kryteriach (rozwiązuje kontradykcję, wykonalność, koszt, zgodność z ograniczeniami problemu) — każde kryterium ma wagę.",
        points: &[
            "Wynik łączny = suma ważona ocen.",
            "Każda ocena ma pisemne uzasadnienie w trailu.",
        ],
        ordered: false,
        how: "wagi, suma, argmax wyboru · LLM: oceny z uzasadnieniem",
    },
    DocCard {
        tag: "E-02 · względem odniesienia",
        title: "Macierz Pugha",
        body: "Klasyka oceny koncepcji w R&D: jeden kandydat zostaje punktem odniesienia (datum), a resztę porównuje się do niego w każdym kryterium.",
        points: &[
            "Oceny tylko: lepszy (+), równy (0), gorszy (−).",
            "Wynik = suma plusów i minusów względem datum.",
        ],
        ordered: false,
        how: "zliczanie +/0/−, ranking · LLM: werdykt per porównanie",
    },
    DocCard {
        tag: "E-03 · każdy z każdym",
        title: "Porównanie parami",
        body: "Najprostsza w interpretacji: każdy kandydat staje w szranki z każdym innym, a w pojedynku wygrywa ten, który lepiej odpowiada na problem.",
        points: &[
            "Przy n kandydatach: n·(n−1)/2 pojedynków.",
            "Ranking = liczba wygranych pojedynków.",
        ],
        ordered: false,
        how: "harmonogram par, zliczanie wygranych · LLM: rozstrzygnięcie pojedynku",
    },
];

fn doc_card(card: &DocCard) -> Element {
    rsx! {
        article { class: "doc-card",
            span { class: "doc-card__tag", "{card.tag}" }
            h3 { "{card.title}" }
            p { "{card.body}" }
            if card.ordered {
                ol {
                    for point in card.points {
                        li { "{point}" }
                    }
                }
            } else {
                ul {
                    for point in card.points {
                        li { "{point}" }
                    }
                }
            }
            p { class: "doc-card__how",
                b { "KOD:" }
                " {card.how}"
            }
        }
    }
}

#[component]
pub fn MethodsPage() -> Element {
    rsx! {
        p { class: "eyebrow", "Jak system myśli" }
        h1 { "Metodologia" }
        p { class: "lead",
            "Heureka nie zgaduje — każdy pomysł ma podpisane źródło, a każda ocena kryterium i uzasadnienie. Poniżej dwie metody generowania rozwiązań i trzy sposoby ich oceny. Czerwonym oznaczamy to, co liczy deterministyczny kod, nie model."
        }

        section { class: "doc-section", aria_labelledby: "gen-h",
            h2 { id: "gen-h", "Metody generowania rozwiązań" }
            div { class: "doc-grid",
                for card in GENERATION.iter() {
                    {doc_card(card)}
                }
            }
        }

        section { class: "doc-section", aria_labelledby: "eval-h",
            h2 { id: "eval-h", "Sposób ewaluacji kandydatów" }
            div { class: "doc-grid",
                for card in EVALUATION.iter() {
                    {doc_card(card)}
                }
            }
        }
    }
}
