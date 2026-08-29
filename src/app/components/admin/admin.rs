use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::{
        self,
        general::UserType::Admin,
        tailwind_constants::{BUTTON, CSV_INPUT, INPUT},
    },
    app::home::{AuthState, actions},
    backend::models::PuzzleSolutions,
};

#[component]
pub fn Login(mut auth: Signal<AuthState>) -> Element {
    rsx!(components::general::Login {
        auth,
        usertype: Admin
    })
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
        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                "Feladat azonosító"
            }
            input { class: "{INPUT}",
                r#type: "text",
                placeholder: "pl. task-1",
                value: "{puzzle_id}",
                oninput: move |evt| puzzle_id.set(evt.value())
            }
        }

        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                "Megoldás"
            }
            input { class: "{INPUT}",
                r#type: "text",
                placeholder: "Helyes válasz",
                value: "{puzzle_solution}",
                oninput: move |evt| puzzle_solution.set(evt.value())
            }
        }

        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                "Pontérték"
            }
            input { class: "{INPUT}",
                r#type: "text",
                placeholder: "pl. 100",
                value: "{puzzle_value}",
                oninput: move |evt| puzzle_value.set(evt.value())
            }
        }

        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                "CSV import"
            }
            input { class: "{CSV_INPUT}",
                r#type: "file",
                r#accept: ".csv",
                onchange: actions::handle_csv(parsed_puzzles, toast_api),
            }
        }

        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                "Admin jelszó"
            }
            input { class: "{INPUT}",
                r#type: "password",
                placeholder: "Jelszó megerősítése",
                value: "{auth_current.password}",
                oninput: move |evt| auth.write().password = evt.value()
            }
        }

        div { class: "flex items-end pt-2",
            button { class: "{BUTTON}",
                onclick: actions::handle_admin_submit(auth, puzzle_id, puzzle_value, puzzle_solution, parsed_puzzles, toast_api),
                "Beállítás"
            }
        }
    }
}
