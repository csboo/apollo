use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::{
        self,
        tailwind_constants::{BUTTON, FLASH, INPUT},
    },
    app::home::{AuthState, actions},
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
};

#[component]
pub fn Login(mut auth: Signal<AuthState>) -> Element {
    auth.write().is_admin = false;
    rsx!(components::general::Login { auth })
}

#[component]
pub fn TaskManager(
    mut auth: Signal<AuthState>,
    puzzle_id: Signal<String>,
    puzzle_solution: Signal<String>,
    mut teams_state: Signal<Vec<(String, SolvedPuzzles)>>,
    puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
) -> Element {
    let auth_current = auth.read().clone();
    let teams = teams_state.read();
    let ref_puzzles = puzzles.read();
    let toast_api = use_toast();

    let solved = teams
        .iter()
        .find(|(team, _)| team == &auth_current.username)
        .map(|(_, solved)| solved);

    let selectopts = solved
        .into_iter()
        .flat_map(|solved| ref_puzzles.iter().filter(|(id, _)| !solved.contains(id)));

    rsx! {
        select {
            class: "{INPUT}",
            cursor: "pointer",
            onchange: move |evt: Event<FormData>| {
                debug!("{}", evt.value());
                puzzle_id.set(evt.value());
            },
            if puzzle_id.is_empty() {
                option { disabled: true, selected: true, "Feladat kiválasztása" }
            }
            for (id, _) in selectopts {
                option {
                    cursor: "pointer",
                    value: "{id}",
                    "{id}"
                }
            }
        }

        input { class: "{INPUT}",
            r#type: "text",
            placeholder: "Megoldás",
            value: "{puzzle_solution}",
            cursor: "text",
            oninput: move |evt| puzzle_solution.set(evt.value())
        }

        button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_user_submit(puzzle_id, puzzle_solution, toast_api), "Küldés" }
    }
}
