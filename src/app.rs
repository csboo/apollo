use dioxus::prelude::*;

mod actions;
mod models;
mod utils;

use crate::{
    app::{
        models::{AuthState, Message},
        utils::parse_puzzle_csv,
    },
    backend::models::{PuzzleId, PuzzleSolutions, PuzzleValue, SolvedPuzzles},
    components::score_table::ScoreTable,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const BUTTON: &str = "ml-4 w-30 px-3 py-2 rounded-lg border border-(--dark2) bg-(--middle) text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const INPUT: &str = "w-50 px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const CSV_INPUT: &str = "w-70 px-3 py-2 rounded-lg border border-gray-300 bg-gray-100 text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";

#[component]
pub fn App() -> Element {
    trace!("kicking off app");
    // State management variables
    trace!("initing variables");
    let mut puzzle_id = use_signal(|| String::new());
    let mut puzzle_solution = use_signal(|| String::new());
    let mut puzzle_value = use_signal(|| String::new());
    let mut auth = use_signal(|| AuthState {
        username: String::new(),
        password: String::new(),
        joined: false,
        is_admin: false,
        show_password_prompt: false,
    });
    let auth_current = auth.read();
    let mut teams_state = use_signal(|| Vec::<(PuzzleId, SolvedPuzzles)>::new());
    let mut puzzles = use_signal(|| Vec::<(PuzzleId, PuzzleValue)>::new());
    let mut message = use_signal(|| None::<(Message, String)>);
    let mut title = use_signal(|| None::<String>);
    let mut is_fullscreen = use_signal(|| false);
    let mut parsed_puzzles = use_signal(|| PuzzleSolutions::new());
    trace!("variables inited");

    // side effect handlers
    use_future(move || async move {
        title.set(
            crate::backend::endpoints::event_title()
                .await
                .inspect_err(|e| message.set(Some((Message::MsgErr, format!("Error: {}", e)))))
                .ok()
                .unwrap_or("Apollo esemény".to_string())
                .into(),
        );
    });

    use_future(move || async move {
        // Call the stream endpoint to get a stream of events
        trace!("calling state_stream");
        let mut stream = crate::backend::endpoints::state_stream().await?;
        trace!("got stream");

        // Then poll it for new events
        while let Some(Ok((new_team_state, new_puzzles))) = stream.next().await {
            trace!("got new data");
            let mut temp_p: Vec<(PuzzleId, PuzzleValue)> = new_puzzles.into_iter().collect();
            temp_p.sort();
            let mut temp_t: Vec<(PuzzleId, SolvedPuzzles)> = new_team_state.into_iter().collect();
            temp_t.sort_by(|a, b| {
                b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)) // solved size, abc order if equal
            });

            puzzles.set(temp_p);
            teams_state.set(temp_t);
            trace!("set new data");
        }

        dioxus::Ok(())
    });

    use_effect(move || {
        if message.read().is_some() {
            // hide after 5 seconds
            spawn(async move {
                gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                message.set(None);
            });
        }
    });

    // action handlers
    let handle_csv = move |evt: Event<FormData>| async move {
        let text = evt
            .files()
            .iter()
            .next()
            .unwrap()
            .read_string()
            .await
            .unwrap();

        parsed_puzzles.set(parse_puzzle_csv(&text));
    };

    let toggle_fullscreen = move |_| {
        trace!("fullscreen toggle called");
        let fullscreen_current = *is_fullscreen.read();
        is_fullscreen.set(!fullscreen_current);
    };

    let handle_action = move |_| async move {
        trace!("action handler called");
        let username_current = auth.read().username.clone();
        let password_current = auth.read().password.clone();

        if !auth.read().joined {
            actions::handle_join(username_current, password_current, &mut auth, &mut message).await;
        } else if auth.read().is_admin {
            actions::handle_admin_submit(
                &mut puzzle_id,
                &mut puzzle_value,
                &mut puzzle_solution,
                &parsed_puzzles,
                password_current,
                &mut message,
            )
            .await;
        } else {
            actions::handle_user_submit(
                &mut puzzle_id,
                &mut puzzle_solution,
                username_current,
                &mut message,
            )
            .await;
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: if *is_fullscreen.read() { "table-only" } else { "normal" },
            div { class: "others-container",
                if let Some(t) = &*title.read() {
                    h1 { class: "mb-4 font-bold text-lg",
                        "{t}",
                    }
                } else {
                    h1 { class: "mb-4 font-bold text-md",
                        "Betöltés..."
                    }
                }

                // Input section
                div { class: "input-section",
                    if !auth_current.joined {
                        // Join form
                        input { class: INPUT,
                            r#type: "text",
                            placeholder: "Csapatnév",
                            value: "{auth_current.username}",
                            oninput: move |evt| auth.write().username = evt.value()
                        }

                        if auth_current.show_password_prompt {
                            input { class: "ml-4 {INPUT}",
                                r#type: "password",
                                placeholder: "Admin jelszó",
                                value: "{auth_current.password}",
                                oninput: move |evt| auth.write().password = evt.value()
                            }
                        }

                        button { class: BUTTON, onclick: handle_action, "Belépés" }
                    } else {
                        // Submit form
                        input { class: INPUT,
                            r#type: "text",
                            placeholder: "Puzzle ID",
                            value: "{puzzle_id}",
                            oninput: move |evt| puzzle_id.set(evt.value())
                        }

                        input { class: "ml-4 {INPUT}",
                            r#type: "text",
                            placeholder: "Megoldás",
                            value: "{puzzle_solution}",
                            oninput: move |evt| puzzle_solution.set(evt.value())
                        }

                        if auth_current.is_admin {
                            input { class: "ml-4 {INPUT}",
                                r#type: "text",
                                placeholder: "Érték/Nyeremény",
                                value: "{puzzle_value}",
                                oninput: move |evt| puzzle_value.set(evt.value())
                            }

                            input { class: "ml-4 {INPUT}",
                                r#type: "password",
                                placeholder: "Admin jelszó",
                                value: "{auth_current.password}",
                                oninput: move |evt| auth.write().password = evt.value()
                            }

                            input { class: "ml-4 {CSV_INPUT}",
                                r#type: "file",
                                r#accept: ".csv",
                                onchange: handle_csv,
                            }

                            button { class: BUTTON, onclick: handle_action, "Beállítás" }
                        } else {
                            button { class: BUTTON, onclick: handle_action, "Küldés" }
                        }

                    }
                }

                // Message popup
                if let Some(m) = &*message.read() {
                    div {
                        class: "popup",
                        id: match m.0 {
                            Message::MsgNorm => {
                                "msgnorm"
                            },
                            Message::MsgErr => {
                                "msgerr"
                            },
                        },
                        "{m.1}"
                    }
                }
            }
            // Teams and puzzles table

            div { class: "table-container",
                ScoreTable {
                    puzzles: puzzles,
                    teams_state: teams_state,
                    toggle_fullscreen: toggle_fullscreen,
                }
            }
        }
    }
}
