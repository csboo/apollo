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
    username_current: String,
    password_current: String,
    auth: &mut Signal<AuthState>,
    message: &mut Signal<Option<(Message, String)>>,
) {
    if check_admin_username(username_current.clone())
        .await
        .is_ok_and(|x| x)
    {
        auth.write().is_admin = true;
        auth.write().show_password_prompt = true;

        // If password is empty, don't proceed yet
        if password_current.is_empty() {
            popup_normal(message, "Adja meg az admin jelszót");
            return;
        }
        auth.write().joined = true;
        return;
    };

    match crate::backend::endpoints::join(username_current.clone()).await {
        Ok(msg) => {
            popup_normal(message, msg);
            auth.write().joined = true;
            auth.write().password = String::new();
            auth.write().show_password_prompt = false;
        }
        Err(e) => {
            popup_error(message, format!("Hiba: {}", e));
        }
    }
}

pub async fn handle_user_submit(
    puzzle_id: &mut Signal<String>,
    puzzle_solution: &mut Signal<String>,
    username_current: String,
    message: &mut Signal<Option<(Message, String)>>,
) {
    let puzzle_current = puzzle_id.read().clone();
    let solution_current = puzzle_solution.read().clone();
    match crate::backend::endpoints::submit_solution(
        username_current,
        puzzle_current,
        solution_current,
    )
    .await
    {
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
    let puzzle_current = puzzle_id.read().clone();
    let solution_current = puzzle_solution.read().clone();
    let Ok(value_current) = puzzle_value.read().parse() else {
        popup_error(message, "Az érték csak szám lehet");
        return;
    };

    match crate::backend::endpoints::set_solution(
        if parsed_puzzles.read().is_empty() {
            PuzzleSolutions::from([(
                puzzle_current,
                Puzzle {
                    value: value_current,
                    solution: solution_current,
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
