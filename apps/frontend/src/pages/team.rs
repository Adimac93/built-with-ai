//! Team bios (Angular: TeamPage). Content ported verbatim.

use dioxus::prelude::*;

struct TeamMember {
    photo: Asset,
    photo_alt: &'static str,
    role: &'static str,
    name: &'static str,
    bio: &'static str,
    tags: &'static [&'static str],
}

fn members() -> [TeamMember; 3] {
    [
        TeamMember {
            photo: asset!("/assets/img/AdmaM.jpeg"),
            photo_alt: "Adam Maciejczuk w czapce z daszkiem, selfie na górskim szlaku",
            role: "Full-stack",
            name: "Adam Maciejczuk",
            bio: "Skleja silnik z interfejsem i pilnuje, żeby demo działało jedną komendą.",
            tags: &["pipeline", "NestJS", "integracja"],
        },
        TeamMember {
            photo: asset!("/assets/img/AdamK.png"),
            photo_alt: "Adam Korwin przy palmie, w tle panorama miasta z wieżą kościoła",
            role: "Frontend",
            name: "Adam Korwin",
            bio: "Odpowiada za arkusz, który właśnie oglądasz — od tokenów po dostępność z klawiatury.",
            tags: &["Angular", "ng-diagram", "a11y"],
        },
        TeamMember {
            photo: asset!("/assets/img/AntekP.jpg"),
            photo_alt: "Antoni Pszenica w okularach, selfie na tle gotyckiej katedry",
            role: "LLM · evals",
            name: "Antoni Pszenica",
            bio: "Trzyma model na krótkiej smyczy schematów, a jakość odpowiedzi mierzy metrykami, nie okiem.",
            tags: &["prompty", "TRIZ", "metryki"],
        },
    ]
}

#[component]
pub fn TeamPage() -> Element {
    rsx! {
        p { class: "eyebrow", "Biuro konstrukcyjne" }
        h1 { "Zespół" }
        p { class: "lead",
            "Troje ludzi, jeden arkusz. Budujemy silnik, który myśli krokami — i każdy z tych kroków ktoś z nas trzyma w ryzach."
        }

        div { class: "crew",
            for member in members() {
                article { class: "person",
                    div { class: "person__portrait",
                        img { src: member.photo, alt: "{member.photo_alt}" }
                    }
                    div { class: "person__body",
                        p { class: "person__role", "{member.role}" }
                        h2 { class: "person__name", "{member.name}" }
                        p { "{member.bio}" }
                        div { class: "person__tags",
                            for tag in member.tags {
                                span { "{tag}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
