#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use dioxus::prelude::*;

pub mod actions;
mod hooks;
mod models;
pub mod utils;

pub use crate::app::models::AuthState;

use crate::{
    backend::models::{PuzzleId, PuzzleSolutions, PuzzleValue, SolvedPuzzles},
    components::{
        input_section::InputSection, score_table::ScoreTable, team_section::TeamSection,
        toast::ToastProvider,
    },
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_CSS: Asset = asset!("/assets/dx-components-theme.css");

#[component]
pub fn App() -> Element {
    trace!("kicking off app");
    // State management variables
    trace!("initing variables");
    let puzzle_id = use_signal(String::new);
    let puzzle_solution = use_signal(String::new);
    let puzzle_value = use_signal(String::new);
    let auth = use_signal(AuthState::default);
    let auth_current = auth.read();
    let teams_state = use_signal(Vec::<(String, SolvedPuzzles)>::new);
    let puzzles = use_signal(Vec::<(PuzzleId, PuzzleValue)>::new);
    let title = use_signal(|| None::<String>);
    let is_fullscreen = use_signal(|| false);
    let parsed_puzzles = use_signal(PuzzleSolutions::new);
    let logout_alert = use_signal(|| false);
    let delete_alert = use_signal(|| false);

    trace!("variables inited");

    // side effect handlers
    hooks::check_auth(auth);
    hooks::load_title(title);
    hooks::subscribe_stream(teams_state, puzzles);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DX_CSS }

        ToastProvider {
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

                div { class: "table-container mt-5 overflow-x-auto", style: "-webkit-overflow-scrolling: touch;",
                    ScoreTable {
                        puzzles: puzzles,
                        teams_state: teams_state,
                        toggle_fullscreen: actions::toggle_fullscreen(is_fullscreen),
                    }
                } // div: table-container

                div { class: "others-container mt-5",
                    if title.read().as_ref().is_some_and(|t| !t.is_empty()) {
                        // Input section
                        div { class: "input-section relative input-flexy-boxy flex flex-wrap gap-3 flex-row",
                            InputSection {
                                auth,
                                puzzle_id,
                                puzzle_value,
                                puzzle_solution,
                                parsed_puzzles,
                                teams_state,
                                puzzles,
                            }
                        } // div: input-section

                        if auth_current.joined && !auth_current.is_admin{
                            TeamSection {
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
}
