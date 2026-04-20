use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::{
        self,
        tailwind_constants::{BUTTON, CSV_INPUT, FLASH, INPUT},
    },
    app::home::{AuthState, actions},
    backend::models::PuzzleSolutions,
};

#[component]
pub fn Login(mut auth: Signal<AuthState>) -> Element {
    auth.write().is_admin = true;
    rsx!(components::general::Login { auth })
}

#[component]
pub fn TaskManager(
    mut auth: Signal<AuthState>,
    puzzle_id: Signal<String>,
    puzzle_value: Signal<String>,
    puzzle_solution: Signal<String>,
    parsed_puzzles: Signal<PuzzleSolutions>,
) -> Element {
    let toast_api = use_toast();
    let auth_current = auth.read().clone();

    rsx! {
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

        input { class: "{CSV_INPUT}",
            r#type: "file",
            r#accept: ".csv",
            cursor: "pointer",
            onchange: actions::handle_csv(parsed_puzzles, toast_api),
        }

    button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_admin_submit(auth, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles, toast_api), "Beállítás" }

    }
}
