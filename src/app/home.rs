#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use dioxus::prelude::*;

pub mod actions;
mod hooks;
mod models;
pub mod utils;

pub use crate::app::home::models::AuthState;

use crate::{
    app::components::{home, loading},
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
};

#[component]
pub fn Home() -> Element {
    trace!("kicking off app");
    trace!("initing variables");
    let puzzle_id = use_signal(String::new);
    let puzzle_solution = use_signal(String::new);
    let auth = use_signal(AuthState::default);
    let teams_state = use_signal(Vec::<(String, SolvedPuzzles)>::new);
    let puzzles = use_signal(Vec::<(PuzzleId, PuzzleValue)>::new);
    let title = use_signal(|| None::<String>);
    let is_fullscreen = use_signal(|| false);
    let logout_alert = use_signal(|| false);
    let delete_alert = use_signal(|| false);

    trace!("variables inited");

    hooks::check_auth(auth);
    hooks::load_title(title);
    hooks::subscribe_stream(teams_state, puzzles);

    if title().is_none() {
        return loading::Loading();
    }

    rsx! {
        div {
            class: if *is_fullscreen.read() { "table-only" } else { "normal min-h-screen" },

            // Header // TODO move into own Element
            div { class: "others-container max-w-5xl mx-auto",
                if let Some(t) = &*title.read() {
                    header { class: "pt-4 pb-8",
                        h1 { class: "text-3xl font-semibold tracking-tight text-(--text-primary)",
                            "{t}"
                        }
                    }
                }
            }

            // Fullscreen table // TODO maybe move into own Element ?
            if is_fullscreen() {
                div { class: "table-overlay",
                    div { class: "table-window",
                        div { class: "table-viewport",
                            home::ScoreTable {
                                puzzles: puzzles,
                                teams_state: teams_state,
                                is_fullscreen: true,
                                toggle_fullscreen: actions::toggle_fullscreen(is_fullscreen),
                            }
                        }
                    }
                }
            } else {
                // Score table preview
                div { class: "max-w-5xl mx-auto",
                    home::ScoreTable {
                        puzzles: puzzles,
                        teams_state: teams_state,
                        is_fullscreen: false,
                        toggle_fullscreen: actions::toggle_fullscreen(is_fullscreen),
                    }
                }
            }

            // Main content // TODO move into own Element
            div { class: "others-container max-w-5xl mx-auto mt-8 space-y-6",
                if auth().joined {
                    // Submit card
                    section { class: "bg-(--bg-elevated) rounded-xl border border-(--border-subtle) p-6",
                        div { class: "mb-5",
                            h2 { class: "text-lg font-medium text-(--text-primary)",
                                "Megoldás beküldése"
                            }
                        }
                        div { class: "flex flex-wrap gap-4 items-end",
                            home::TaskManager {
                                auth,
                                puzzle_id,
                                puzzle_solution,
                                teams_state,
                                puzzles,
                            }
                        }
                    }

                    home::TeamSection {
                        auth,
                        logout_alert,
                        delete_alert,
                        teams_state,
                        puzzles,
                    }
                } else {
                    // Login card
                    section { class: "bg-(--bg-elevated) rounded-xl border border-(--border-subtle) p-6",
                        div { class: "mb-5",
                            h2 { class: "text-lg font-medium text-(--text-primary)",
                                "Bejelentkezés"
                            }
                        }
                        div { class: "flex flex-wrap gap-4 items-end",
                                home::Login { auth }
                        }
                    }
                }
            }
        }
    }
}
