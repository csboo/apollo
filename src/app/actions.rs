use dioxus::{prelude::*, signals::Signal};

use crate::{
    app::{
        models::{AuthState, Message},
        utils::{
            parse_puzzle_csv, popup_error, popup_normal, validate_puzzle_id,
            validate_puzzle_solution, validate_puzzle_value,
        },
    },
    backend::models::{Puzzle, PuzzleSolutions},
};

// TODO could be handled better
fn check_admin_username(username: String) -> bool {
    // use std::env;
    let admin_username = "admin";
    username == admin_username
}

pub async fn handle_join(mut auth: Signal<AuthState>, message: Signal<Option<(Message, String)>>) {
    let u = auth.read().username.clone();
    if !auth.read().validate_username(message) {
        return;
    }

    if check_admin_username(u.clone()) {
        auth.write().is_admin = true;
        auth.write().show_password_prompt = true;

        // If password is empty, don't proceed yet
        if !auth.read().validate_password(message) {
            return;
        }
        let _ignored_result =
            crate::backend::endpoints::set_admin_password(auth.read().password.clone()).await;
        auth.write().joined = true;
        return;
    };

    let _ok_none = crate::backend::endpoints::join(u.clone()).await;
    match crate::backend::endpoints::auth_state().await {
        Ok(uname) => {
            popup_normal(message, format!("Üdv, {}", uname));
            auth.write().joined = true; // TODO auth.reset(_somefield)
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
    mut puzzle_id: Signal<String>,
    mut puzzle_solution: Signal<String>,
    message: Signal<Option<(Message, String)>>,
) {
    let puzzle_current = puzzle_id.read().clone();
    let solution_current = puzzle_solution.read().clone();
    if !validate_puzzle_id(&puzzle_current, message) {
        return;
    }
    if !validate_puzzle_solution(&solution_current, message) {
        return;
    }

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
    mut puzzle_id: Signal<String>,
    mut puzzle_value: Signal<String>,
    mut puzzle_solution: Signal<String>,
    parsed_puzzles: Signal<PuzzleSolutions>,
    password_current: String,
    message: Signal<Option<(Message, String)>>,
) {
    match crate::backend::endpoints::set_solution(
        if parsed_puzzles.read().is_empty() {
            if !validate_puzzle_id(&puzzle_id.read().clone(), message) {
                return;
            }
            if !validate_puzzle_solution(&puzzle_solution.read().clone(), message) {
                return;
            }
            if !validate_puzzle_value(&puzzle_value.read().clone(), message) {
                return;
            }

            debug!("parsed puzzles is empty, trying from manual values");
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

pub fn handle_csv(
    mut parsed_puzzles: Signal<PuzzleSolutions>,
    message: Signal<Option<(Message, String)>>,
) -> impl FnMut(Event<FormData>) + 'static {
    move |form_data| {
        spawn(async move {
            if let Some(file) = form_data.files().first() {
                let Ok(text) = file.read_string().await else {
                    warn!("couldn't parse text from selected file");
                    return;
                };
                parsed_puzzles.set(parse_puzzle_csv(&text, message));
                debug!("set puzzles from csv");
            } else {
                warn!("couldn't read selected file");
            };
        });
    }
}

pub fn toggle_fullscreen(
    mut is_fullscreen: Signal<bool>,
) -> impl FnMut(Event<MouseData>) + 'static {
    move |_| {
        trace!("fullscreen toggle called");
        let fullscreen_current = *is_fullscreen.read();
        is_fullscreen.set(!fullscreen_current);
    }
}

pub fn handle_action(
    auth: Signal<AuthState>,
    message: Signal<Option<(Message, String)>>,
    puzzle_id: Signal<String>,
    puzzle_value: Signal<String>,
    puzzle_solution: Signal<String>,
    parsed_puzzles: Signal<PuzzleSolutions>,
) -> impl FnMut(Event<MouseData>) + 'static {
    move |_| {
        spawn(async move {
            trace!("action handler called");
            if !auth.read().joined {
                self::handle_join(auth, message).await;
            } else if auth.read().is_admin {
                self::handle_admin_submit(
                    puzzle_id,
                    puzzle_value,
                    puzzle_solution,
                    parsed_puzzles,
                    auth.read().password.clone(),
                    message,
                )
                .await;
            } else {
                self::handle_user_submit(puzzle_id, puzzle_solution, message).await;
            }
        });
    }
}

pub fn handle_logout(
    mut auth: Signal<AuthState>,
    message: Signal<Option<(Message, String)>>,
    superlogout: bool,
) -> impl FnMut(Event<MouseData>) + 'static {
    let wipe = match superlogout {
        true => Some(true),
        false => None,
    };
    move |_| {
        spawn(async move {
            match crate::backend::endpoints::logout(wipe).await {
                Ok(_) => {
                    popup_normal(message, format!("Viszlát, {}", auth.read().username));
                    auth.set(AuthState::default());
                }
                Err(e) => {
                    popup_error(
                        message,
                        format!("Hiba: {}", e.message.unwrap_or("ismeretlen hiba".into())),
                    );
                }
            }
        });
    }
}
