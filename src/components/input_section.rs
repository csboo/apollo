use dioxus::prelude::*;

use crate::{
    app::{AuthState, Message, actions},
    backend::models::{PuzzleId, PuzzleSolutions, PuzzleValue, SolvedPuzzles},
    components::tailwind_constants::{BUTTON, CSV_INPUT, FLASH, INPUT},
};

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
                input { class: "{INPUT}",
                    r#type: "password",
                    placeholder: "Admin jelszó",
                    value: "{auth_current.password}",
                    cursor: "text",
                    oninput: move |evt| auth.write().password = evt.value()
                }
            }

            button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_action(auth, message, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Belépés" }
        } else {
        // Submit form
            if !auth_current.is_admin {
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
                            "{id}. feladat"
                        }
                    }
                }
            } else {
                input { class: "{INPUT}",
                    r#type: "text",
                    placeholder: "Feladat",
                    value: "{puzzle_id}",
                    cursor: "text",
                    oninput: move |evt| puzzle_id.set(evt.value())
                }
            }

            input { class: "{INPUT}",
                r#type: "text",
                placeholder: "Megoldás",
                value: "{puzzle_solution}",
                cursor: "text",
                oninput: move |evt| puzzle_solution.set(evt.value())
            }

            if auth_current.is_admin {
                input { class: "{INPUT}",
                    r#type: "text",
                    placeholder: "Érték/Nyeremény",
                    value: "{puzzle_value}",
                    cursor: "text",
                    oninput: move |evt| puzzle_value.set(evt.value())
                }

                input { class: "{INPUT}",
                    r#type: "password",
                    placeholder: "Admin jelszó",
                    value: "{auth_current.password}",
                    cursor: "text",
                    oninput: move |evt| auth.write().password = evt.value()
                }

                input { class: "{CSV_INPUT}",
                    r#type: "file",
                    r#accept: ".csv",
                    cursor: "pointer",
                    onchange: actions::handle_csv(parsed_puzzles, message),
                }

                button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_action(auth, message, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Beállítás" }
            } else {
                button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_action(auth, message, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Küldés" }
            }
    })
}
