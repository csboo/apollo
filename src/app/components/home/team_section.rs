use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::{alert_dialog::*, home::team_status::*, tailwind_constants::BUTTON_RED},
    app::home::{AuthState, actions::handle_logout, utils::get_points_of},
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
};

#[component]
pub fn TeamSection(
    auth: Signal<AuthState>,
    mut logout_alert: Signal<bool>,
    mut delete_alert: Signal<bool>,
    mut teams_state: Signal<Vec<(String, SolvedPuzzles)>>,
    puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
) -> Element {
    // let auth_current = auth.read().clone();
    let toast_api = use_toast();
    let points = teams_state
        .read()
        .iter()
        .find(|(team, _)| *team == auth().username)
        .map(|team| get_points_of(team, puzzles()))
        .unwrap_or(0);

    rsx! {
        // Team status
        TeamStatus {
            team: auth().username,
            points: points,
        }

        // Action buttons
        div { class: "flex gap-3",
            button { class: "{BUTTON_RED}",
                onclick: move |_| logout_alert.set(true),
                "Kijelentkezés"
            }

            button { class: "{BUTTON_RED} opacity-70 hover:opacity-100",
                onclick: move |_| delete_alert.set(true),
                "Csapat törlése"
            }
        }

        // Logout dialog
        AlertDialogRoot { open: logout_alert(), on_open_change: move |v| logout_alert.set(v),
            AlertDialogContent {
                AlertDialogTitle { "Kijelentkezés" }
                AlertDialogDescription {
                    "Biztosan ki szeretnél lépni?"
                    br {}
                    span { class: "text-(--text-muted)",
                        "Később visszaléphetsz, a progresszió megmarad."
                    }
                }
                AlertDialogActions {
                    AlertDialogCancel { "Mégsem" }
                    AlertDialogAction { on_click: handle_logout(auth, toast_api, false), "Kilépés" }
                }
            }
        }

        // Delete dialog
        AlertDialogRoot { open: delete_alert(), on_open_change: move |v| delete_alert.set(v),
            AlertDialogContent {
                AlertDialogTitle { "Csapat törlése" }
                AlertDialogDescription {
                    "Ez véglegesen törli a csapat minden adatát."
                }
                AlertDialogActions {
                    AlertDialogCancel { "Mégsem" }
                    AlertDialogAction { on_click: handle_logout(auth, toast_api, true), "Törlés" }
                }
            }
        }
    }
}
