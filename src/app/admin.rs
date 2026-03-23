#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use crate::app::components::admin;
use crate::app::home::AuthState;
use crate::backend::models::PuzzleSolutions;
use dioxus::prelude::*;

#[component]
pub fn Admin() -> Element {
    let auth = use_signal(AuthState::default);
    let auth_current = auth.read();
    let puzzle_id = use_signal(String::new);
    let puzzle_solution = use_signal(String::new);
    let puzzle_value = use_signal(String::new);
    let parsed_puzzles = use_signal(PuzzleSolutions::new);

    rsx! {
        h1 { class: "mb-4 font-bold text-lg",
            "Apollo admin panel",
        }

        div { class: "input-section relative input-flexy-boxy flex flex-wrap gap-3 flex-row",
            if !auth_current.joined {
                admin::Login { auth }
            } else {
                // Submit form
                admin::TaskManager {
                    auth,
                    puzzle_id,
                    puzzle_value,
                    puzzle_solution,
                    parsed_puzzles,
                 }
            } // else
        }
    }
}
