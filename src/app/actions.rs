use dioxus::{prelude::*, signals::Signal};

use crate::{
    app::{
        models::{AuthState, Message},
        utils::{popup_error, popup_normal},
    },
    backend::models::{Puzzle, PuzzleSolutions},
};

// TODO could be handeled in much better ways
async fn check_admin_username(username: String) -> Result<bool, ServerFnError> {
    // use std::env;
    let admin_username = "jani";
    Ok(username == admin_username)
}

pub async fn handle_join(
    auth: &mut Signal<AuthState>,
    message: &mut Signal<Option<(Message, String)>>,
) {
    let u = auth.read().username.clone();
    if check_admin_username(u.clone()).await.is_ok_and(|x| x) {
        auth.write().is_admin = true;
        auth.write().show_password_prompt = true;

        // If password is empty, don't proceed yet
        if auth.read().password.is_empty() {
            popup_normal(message, "Adja meg az admin jelszót");
            return;
        }
        auth.write().joined = true;
        return;
    };

    match crate::backend::endpoints::join(u.clone()).await {
        Ok(_) => {
            popup_normal(message, format!("Üdv, {}", u));
            auth.write().joined = true;
            auth.write().password = String::new();
            auth.write().show_password_prompt = false;
        }
        Err(e) => {
            popup_error(
                message,
                format!("Hiba: {}", e.message.unwrap_or("ismeretlen hiba".into())),
            );
        }
    }
}

pub async fn handle_user_submit(
    puzzle_id: &mut Signal<String>,
    puzzle_solution: &mut Signal<String>,
    message: &mut Signal<Option<(Message, String)>>,
) {
    let puzzle_current = puzzle_id.read().clone();
    let solution_current = puzzle_solution.read().clone();
    match crate::backend::endpoints::submit_solution(puzzle_current, solution_current).await {
        Ok(msg) => {
            popup_normal(message, msg);
            puzzle_id.set(String::new());
            puzzle_solution.set(String::new());
        }
        Err(e) => {
            popup_error(message, format!("Hiba: {}", e));
        }
    }
}

pub async fn handle_admin_submit(
    puzzle_id: &mut Signal<String>,
    puzzle_value: &mut Signal<String>,
    puzzle_solution: &mut Signal<String>,
    parsed_puzzles: &Signal<PuzzleSolutions>,
    password_current: String,
    message: &mut Signal<Option<(Message, String)>>,
) {
    // Submit solution - call backend function directly
    match crate::backend::endpoints::set_solution(
        if parsed_puzzles.read().is_empty() {
            let Ok(value_current) = puzzle_value.read().parse() else {
                popup_error(message, "Az érték csak szám lehet");
                return;
            };
            PuzzleSolutions::from([(
                puzzle_id.read().clone(),
                Puzzle {
                    value: value_current,
                    solution: puzzle_solution.read().clone(),
                },
            )])
        } else {
            parsed_puzzles.read().clone()
        },
        password_current,
    )
    .await
    {
        Ok(msg) => {
            popup_normal(message, msg);
            puzzle_id.set(String::new());
            puzzle_solution.set(String::new());
            puzzle_value.set(String::new());
            // password.set(String::new()); NOTE should remember password?
        }
        Err(e) => {
            popup_error(
                message,
                format!("Hiba: {}", e.message.unwrap_or("ismeretlen hiba".into())),
            );
        }
    }
}
