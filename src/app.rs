#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use dioxus::prelude::*;

pub mod actions;
mod hooks;
mod models;
mod utils;

const BUTTON: &str = "w-30 px-3 py-2 rounded-lg border border-red-900 bg-red-400 text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-red-500 focus:border-red-500 transition";
pub mod utils;

pub use crate::app::models::{AuthState, Message};

use crate::{
    app::actions::handle_logout,
    backend::models::{PuzzleId, PuzzleSolutions, PuzzleValue, SolvedPuzzles},
    components::{
        input_section::InputSection, message_popup::MessagePopup, score_table::ScoreTable,
        team_status::TeamStatus,
    },
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
    let message = use_signal(|| None::<(Message, String)>);
    let title = use_signal(|| None::<String>);
    let is_fullscreen = use_signal(|| false);
    let parsed_puzzles = use_signal(PuzzleSolutions::new);
    trace!("variables inited");

    // side effect handlers
    hooks::auto_hide_message(message);
    hooks::check_auth(auth, message);
    hooks::load_title(title, message);
    hooks::subscribe_stream(teams_state, puzzles);

    let teams = teams_state.read();
    let ref_puzzles = puzzles.read();

    let solved = teams
        .iter()
        .find(|(team, _)| team == &auth_current.username)
        .map(|(_, solved)| solved);

    let points: u32 = solved
        .as_ref()
        .map(|solved| {
            ref_puzzles
                .iter()
                .filter(|(id, _)| solved.contains(id))
                .map(|(_, value)| *value)
                .sum()
        })
        .unwrap_or(0);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

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
                            message,
                            puzzle_id,
                            puzzle_value,
                            puzzle_solution,
                            parsed_puzzles,
                            teams_state,
                            puzzles,
                        }
                    } // div: input-section

                    if auth_current.joined && !auth_current.is_admin{
                        div { class: "mt-5",
                            TeamStatus {
                                team: auth_current.username.clone(),
                                points: points,
                            }
                        }
                        div { class: "mt-5",
                            button { class: "{BUTTON}",
                                onclick: handle_logout(auth, message), // TODO support wipe logout
                                cursor: "pointer",
                                "Kijelentkezés"
                            }
                        }
                    }

                    // Message popup
                    if let Some(m) = &*message.read() {
                        MessagePopup {
                            level: m.0.clone(),
                            text: m.1.clone(),
                        }
                    } // end message
                }
            } // div: other-container
        } // end main div
    }
}
