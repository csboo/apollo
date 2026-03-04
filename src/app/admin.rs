#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use crate::app::home::actions;
use crate::backend::models::PuzzleSolutions;
use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::app::home::AuthState;
use crate::components::tailwind_constants::{BUTTON, CSV_INPUT, FLASH, INPUT};

#[component]
pub fn Admin() -> Element {
    let mut auth = use_signal(AuthState::default);
    let auth_current = auth.read();
    let mut puzzle_id = use_signal(String::new);
    let mut puzzle_solution = use_signal(String::new);
    let mut puzzle_value = use_signal(String::new);
    let toast_api = use_toast();
    let parsed_puzzles = use_signal(PuzzleSolutions::new);

    rsx! {
        div { class: "input-section relative input-flexy-boxy flex flex-wrap gap-3 flex-row",
            if !auth_current.joined {
                // Join form
                input { class: INPUT,
                    r#type: "text",
                    placeholder: "Admin név",
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

                button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_action(auth, toast_api, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Belépés" }
            } else {
            // Submit form
                input { class: "{INPUT}",
                    r#type: "text",
                    placeholder: "Feladat",
                    value: "{puzzle_id}",
                    cursor: "text",
                    oninput: move |evt| puzzle_id.set(evt.value())
                }

                input { class: "{INPUT}",
                    r#type: "text",
                    placeholder: "Megoldás",
                    value: "{puzzle_solution}",
                    cursor: "text",
                    oninput: move |evt| puzzle_solution.set(evt.value())
                }

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
                    onchange: actions::handle_csv(parsed_puzzles, toast_api),
                }

                button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_action(auth, toast_api, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles), "Beállítás" }
            } // else
        }
    }
}
