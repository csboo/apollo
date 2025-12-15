use dioxus::prelude::*;

use crate::app::{AuthState, Message, actions::handle_logout, utils::get_points_of};
use crate::backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles};
use crate::components::{alert_dialog::*, tailwind_constants::BUTTON, team_status::TeamStatus};

#[component]
pub fn TeamSection(
    auth: Signal<AuthState>,
    message: Signal<Option<(Message, String)>>,
    mut logout_alert: Signal<bool>,
    mut delete_alert: Signal<bool>,
    mut teams_state: Signal<Vec<(String, SolvedPuzzles)>>,
    puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
) -> Element {
    let auth_current = auth.read().clone();
    let points = teams_state
        .read()
        .iter()
        .find(|(team, _)| team == &auth_current.username)
        .map(|team| get_points_of(team, puzzles.read().clone()))
        .unwrap_or(0);

    rsx! {
        div { class: "mt-5",
            TeamStatus {
                team: auth_current.username.clone(),
                points: points,
            }
        }
        div { class: "mt-10",
            button { class: "{BUTTON}",
                onclick: move |_| logout_alert.set(true),
                cursor: "pointer",
                "Kijelentkezés"
            }

            AlertDialogRoot { open: logout_alert(), on_open_change: move |v| logout_alert.set(v),
                AlertDialogContent {
                    AlertDialogTitle { "Delete item" }
                    AlertDialogDescription {
                        "Biztosan ki szeretnél lépni?"
                        br {  }
                        "(Később visszaléphetsz, eddigi progressziód megmarad)"
                    }
                    AlertDialogActions {
                        AlertDialogCancel { "Mégsem" }
                        AlertDialogAction { on_click: handle_logout(auth, message, false), "Kilépés" }
                    }
                }
            }
        }
        div { class: "mt-2",
            button { class: "{BUTTON}",
                onclick: move |_| delete_alert.set(true),
                cursor: "pointer",
                "Csapat törlése"
            }

            AlertDialogRoot { open: delete_alert(), on_open_change: move |v| delete_alert.set(v),
                AlertDialogContent {
                    AlertDialogTitle { "Delete item" }
                    AlertDialogDescription {
                        "Ez a funkció a csapat minden adatát ",
                        strong { "véglegesen törli." }
                        br {}
                        "Biztosan folytatod?"
                    }
                    AlertDialogActions {
                        AlertDialogCancel { "Mégsem" }
                        AlertDialogAction { on_click: handle_logout(auth, message, true), "Csapat Törlése" }
                    }
                }
            }
        }
    }
}
