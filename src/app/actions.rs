use dioxus::{prelude::*, signals::Signal};

use crate::{
    app::models::{AuthState, Message},
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
    if let Ok(is_admin_user) = check_admin_username(username_current.clone()).await {
        if is_admin_user {
            auth.write().is_admin = true;
            auth.write().show_password_prompt = true;

            // If password is empty, don't proceed yet
            if password_current.is_empty() {
                message.set(Some((
                    Message::MsgNorm,
                    "Adja meg az admin jelszót".to_string(),
                )));
                return;
            }
            auth.write().joined = true;
            return;
        }
    };

    match crate::backend::endpoints::join(username_current.clone()).await {
        Ok(msg) => {
            message.set(Some((Message::MsgNorm, msg.clone())));
            auth.write().joined = true;
            auth.write().password = String::new();
            auth.write().show_password_prompt = false;
        }
        Err(e) => {
            message.set(Some((Message::MsgErr, format!("Error: {}", e))));
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
            message.set(Some((Message::MsgNorm, msg)));
            puzzle_id.set(String::new());
            puzzle_solution.set(String::new());
        }
        Err(e) => {
            message.set(Some((Message::MsgErr, format!("Error: {}", e))));
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
    let value_current = puzzle_value.read().clone();
    match crate::backend::endpoints::set_solution(
        if parsed_puzzles.read().is_empty() {
            let value_current_num = value_current.parse::<u32>().unwrap(); // TODO WARN unwrap
            PuzzleSolutions::from([(
                puzzle_current,
                Puzzle {
                    value: value_current_num,
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
            message.set(Some((Message::MsgNorm, msg)));
            puzzle_id.set(String::new());
            puzzle_solution.set(String::new());
            puzzle_value.set(String::new());
            // password.set(String::new()); NOTE should remember password?
        }
        Err(e) => {
            message.set(Some((Message::MsgErr, format!("Error: {}", e))));
        }
    }
}
