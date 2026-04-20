#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use dioxus::prelude::*;

pub mod actions;
mod hooks;
mod models;
pub mod utils;

pub use crate::app::home::models::AuthState;

use crate::{
    app::components::home,
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
};

#[component]
pub fn Home() -> Element {
    trace!("kicking off app");
    // State management variables
    trace!("initing variables");
    let puzzle_id = use_signal(String::new);
    let puzzle_solution = use_signal(String::new);
    let auth = use_signal(AuthState::default);
    let auth_current = auth.read();
    let teams_state = use_signal(Vec::<(String, SolvedPuzzles)>::new);
    let puzzles = use_signal(Vec::<(PuzzleId, PuzzleValue)>::new);
    let title = use_signal(|| None::<String>);
    let is_fullscreen = use_signal(|| false);
    let logout_alert = use_signal(|| false);
    let delete_alert = use_signal(|| false);

    trace!("variables inited");

    // side effect handlers
    hooks::check_auth(auth);
    hooks::load_title(title);
    hooks::subscribe_stream(teams_state, puzzles);

    rsx! {
        div { class: if *is_fullscreen.read() { "table-only" } else { "normal" },
            div { class: "others-container",
                if let Some(t) = &*title.read() {
                    h1 { class: "mb-4 font-bold text-lg",
                        "{t}",
                    }
                } else {
                    div { class: "loading",
                        h1 { class: "font-bold text-[clamp(1rem,4vw,2.5rem)]",
                            "Várakozás az Apollo kiszolgálóra"
                        }
                    }
                }

            } // div: other-container

            div { class: "table-container mt-5", style: "-webkit-overflow-scrolling: touch;",
                home::ScoreTable {
                    puzzles: puzzles,
                    teams_state: teams_state,
                    toggle_fullscreen: actions::toggle_fullscreen(is_fullscreen),
                }
            } // div: table-container

            div { class: "others-container mt-5",
                if title.read().as_ref().is_some_and(|t| !t.is_empty()) {
                    // Input section
                    div { class: "input-section relative input-flexy-boxy flex flex-wrap gap-3 flex-row",
                        if auth_current.joined {
                            home::TaskManager {
                                auth,
                                puzzle_id,
                                puzzle_solution,
                                teams_state,
                                puzzles,
                            }
                        } else {
                            home::Login { auth }
                        }
                    } // div: input-section

                    if auth_current.joined {
                        home::TeamSection {
                            auth,
                            logout_alert,
                            delete_alert,
                            teams_state,
                            puzzles,
                        }

                    }
                }
            } // div: other-container
        } // end main div
    }
}
