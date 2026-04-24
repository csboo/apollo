#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use crate::app::components::admin;
use crate::app::home::AuthState;
use crate::backend::models::PuzzleSolutions;
use dioxus::prelude::*;

#[component]
pub fn Admin() -> Element {
    let auth = use_signal(AuthState::default);
    let puzzle_id = use_signal(String::new);
    let puzzle_solution = use_signal(String::new);
    let puzzle_value = use_signal(String::new);
    let parsed_puzzles = use_signal(PuzzleSolutions::new);

    rsx! {
        div { class: "min-h-screen max-w-4xl mx-auto py-8",
            // Header
            header { class: "mb-8",
                h1 { class: "text-2xl font-semibold tracking-tight text-(--text-primary)",
                    "Admin Panel"
                }
                p { class: "text-(--text-muted) mt-1",
                    "Feladatok és beállítások kezelése"
                }
            }

            // Content card
            section { class: "bg-(--bg-elevated) rounded-xl border border-(--border-subtle) p-6",
                if auth().joined {
                    div { class: "mb-5",
                        h2 { class: "text-lg font-medium text-(--text-primary)",
                            "Feladat beállítása"
                        }
                    }

                    div { class: "flex flex-wrap gap-4 items-end",
                        admin::TaskManager {
                            auth,
                            puzzle_id,
                            puzzle_value,
                            puzzle_solution,
                            parsed_puzzles,
                        }
                    }
                } else {
                    div { class: "mb-5",
                        h2 { class: "text-lg font-medium text-(--text-primary)",
                            "Bejelentkezés"
                        }
                    }

                    div { class: "flex flex-wrap gap-4 items-end",
                        admin::Login { auth }
                    }
                }
            }
        }
    }
}
