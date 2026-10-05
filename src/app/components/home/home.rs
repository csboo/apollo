use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::{
        self,
        general::UserType::Player,
        tailwind_constants::{BUTTON, INPUT, SELECT},
    },
    app::home::{AuthState, actions},
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
};

#[component]
pub fn Login(mut auth: Signal<AuthState>) -> Element {
    rsx!(components::general::Login {
        auth,
        usertype: Player
    })
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
        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                "Feladat"
            }
            select {
                class: "{SELECT}",
                onchange: move |evt: Event<FormData>| {
                    debug!("{}", evt.value());
                    puzzle_id.set(evt.value());
                },
                if puzzle_id.is_empty() {
                    option { disabled: true, selected: true, "Válassz feladatot..." }
                }
                for (id, _) in selectopts {
                    option { value: "{id}", "{id}" }
                }
            }
        }

        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                "Megoldás"
            }
            input { class: "{INPUT}",
                r#type: "text",
                placeholder: "Add meg a megoldást",
                value: "{puzzle_solution}",
                oninput: move |evt| puzzle_solution.set(evt.value())
            }
        }

        div { class: "flex items-end",
            button { class: "{BUTTON}",
                onclick: actions::handle_user_submit(puzzle_id, puzzle_solution, toast_api),
                "Beküldés"
            }
        }
    }
}
