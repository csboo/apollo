use dioxus::prelude::*;

use crate::{
    app::{AuthState, Message, actions},
    backend::models::{PuzzleId, PuzzleSolutions, PuzzleValue, SolvedPuzzles},
};

const BUTTON: &str = "ml-4 w-30 px-3 py-2 rounded-lg border border-(--dark2) bg-(--middle) text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const INPUT: &str = "w-50 px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const CSV_INPUT: &str = "w-70 px-3 py-2 rounded-lg border border-gray-300 bg-gray-100 text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";

#[component]
pub fn InputSection(
    auth: Signal<AuthState>,
    message: Signal<Option<(Message, String)>>,
    puzzle_id: Signal<String>,
    puzzle_value: Signal<String>,
    puzzle_solution: Signal<String>,
    parsed_puzzles: Signal<PuzzleSolutions>,
    mut teams_state: Signal<Vec<(String, SolvedPuzzles)>>,
    puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
) -> Element {
    let auth_current = auth.read().clone();
    let teams = teams_state.read();
    let ref_puzzles = puzzles.read();

    let solved = teams
        .iter()
        .find(|(team, _)| team == &auth_current.username)
        .map(|(_, solved)| solved);

    let selectopts = solved
        .into_iter()
        .flat_map(|solved| ref_puzzles.iter().filter(|(id, _)| !solved.contains(id)));

    // needed for dropdown to have initial value
    if let Some(firstvalid) = selectopts.clone().next().map(|(id, _)| id) {
        puzzle_id.set(firstvalid.to_string());
    }

    rsx!(
        if !auth_current.joined {
        // Join form
        input { class: INPUT,
            r#type: "text",
            placeholder: "Csapatnév",
            value: "{auth_current.username}",
            cursor: "text",
            oninput: move |evt| auth.write().username = evt.value()
        }

        if auth_current.show_password_prompt {
            input { class: "ml-4 {INPUT}",
                r#type: "password",
                placeholder: "Admin jelszó",
                value: "{auth_current.password}",
                cursor: "text",
                oninput: move |evt| auth.write().password = evt.value()
            }
        }

        button { class: BUTTON, cursor: "pointer", onclick: actions::handle_action(auth, message, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Belépés" }
    } else {
        // Submit form
        div { class: "input-flexy-boxy flex flex-row h-[50px]",
            if !auth_current.is_admin {
                select {
                    class: "{INPUT}",
                    cursor: "pointer",
                    onchange: move |evt: Event<FormData>| {
                        debug!("{}", evt.value());
                        puzzle_id.set(evt.value());
                    },
                    for (id, _) in selectopts {
                        option {
                            cursor: "pointer",
                            value: "{id}",
                            "{id}. feladat"
                        }
                    }
                }
            } else {
                input { class: "ml-4 {INPUT}",
                    r#type: "text",
                    placeholder: "Feladat",
                    value: "{puzzle_id}",
                    cursor: "text",
                    oninput: move |evt| puzzle_id.set(evt.value())
                }
            }

            input { class: "ml-4 {INPUT}",
                r#type: "text",
                placeholder: "Megoldás",
                value: "{puzzle_solution}",
                cursor: "text",
                oninput: move |evt| puzzle_solution.set(evt.value())
            }

            if auth_current.is_admin {
                input { class: "ml-4 {INPUT}",
                    r#type: "text",
                    placeholder: "Érték/Nyeremény",
                    value: "{puzzle_value}",
                    cursor: "text",
                    oninput: move |evt| puzzle_value.set(evt.value())
                }

                input { class: "ml-4 {INPUT}",
                    r#type: "password",
                    placeholder: "Admin jelszó",
                    value: "{auth_current.password}",
                    cursor: "text",
                    oninput: move |evt| auth.write().password = evt.value()
                }

                input { class: "ml-4 {CSV_INPUT}",
                    r#type: "file",
                    r#accept: ".csv",
                    cursor: "pointer",
                    onchange: actions::handle_csv(parsed_puzzles.clone(), message.clone()),
                }

                button { class: BUTTON, cursor: "pointer", onclick: actions::handle_action(auth, message, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Beállítás" }
            } else {
                button { class: BUTTON, cursor: "pointer", onclick: actions::handle_action(auth, message, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Küldés" }
            }
        }

    })
}
