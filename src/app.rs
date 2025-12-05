use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};

use crate::{
    backend::{Puzzle, PuzzleSolutions, PuzzlesExisting, TeamsState},
    components::tooltip::*,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const BUTTON: &str = "ml-4 w-30 px-3 py-2 rounded-lg border border-(--dark2) bg-(--middle) text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const INPUT: &str = "w-50 px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";

// TODO could be handeled in much better ways
async fn check_admin_username(username: String) -> Result<bool, ServerFnError> {
    // use std::env;
    let admin_username = "jani";
    Ok(username == admin_username)
}

#[component]
pub fn App() -> Element {
    trace!("kicking off app");
    // State management variables
    trace!("initing variables");
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut puzzle_id = use_signal(|| String::new());
    let mut puzzle_solution = use_signal(|| String::new());
    let mut puzzle_value = use_signal(|| String::new());
    let mut joined = use_signal(|| false);
    let mut is_admin = use_signal(|| false);
    let mut show_password_prompt = use_signal(|| false);
    let mut teams_state = use_signal(|| TeamsState::new());
    let mut puzzles = use_signal(|| PuzzlesExisting::new());
    let mut message = use_signal(|| None::<String>);
    let mut title = use_signal(|| None::<String>);
    trace!("variables inited");

    use_future(move || async move {
        title.set(
            crate::backend::event_title()
                .await
                .inspect_err(|e| message.set(Some(format!("Error: {}", e))))
                .ok(),
        );
    });

    use_effect(move || {
        if message.read().is_some() {
            // hide after 5 seconds
            // let message = message.clone();
            spawn(async move {
                gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                message.set(None);
            });
        }
    });

    use_future(move || async move {
        // Call the stream endpoint to get a stream of events
        trace!("calling state_stream");
        let mut stream = crate::backend::state_stream().await?;
        trace!("got stream");

        // Then poll it for new events
        while let Some(Ok(data)) = stream.next().await {
            trace!("got new data");
            teams_state.set(data.0);
            puzzles.set(data.1);
            trace!("set new data");
        }

        dioxus::Ok(())
    });

    // Handle join/submit button click
    // TODO this is very ugly function thing make it better
    let handle_action = move |_| async move {
        trace!("action handler called");
        let username_current = username.read().clone();
        let password_current = password.read().clone();
        let is_joined = *joined.read();
        let admin = *is_admin.read();

        if !is_joined {
            // Check if username is admin before joining
            if let Ok(is_admin_user) = check_admin_username(username_current.clone()).await {
                if is_admin_user {
                    is_admin.set(true);
                    show_password_prompt.set(true);

                    // If password is empty, don't proceed yet
                    if password_current.is_empty() {
                        message.set(Some("Please enter admin password".to_string()));
                        return;
                    }
                    joined.set(true);
                    return;
                }
            };

            match crate::backend::join(username_current.clone()).await {
                Ok(msg) => {
                    message.set(Some(msg.clone()));
                    joined.set(true);
                    password.set(String::new());
                    show_password_prompt.set(false);
                }
                Err(e) => {
                    message.set(Some(format!("Error: {}", e)));
                }
            }
        } else {
            // Submit solution - call backend function directly
            let puzzle_current = puzzle_id.read().clone();
            let solution_current = puzzle_solution.read().clone();
            let value_current = puzzle_value.read().clone();
            // trace!(
            //     "value is '{}' is_empty: '{}'",
            //     &value_current,
            //     &value_current.is_empty()
            // );
            if admin {
                let value_current_num = value_current.parse::<u32>().unwrap();
                match crate::backend::set_solution(
                    PuzzleSolutions::from([(
                        puzzle_current,
                        Puzzle {
                            value: value_current_num,
                            solution: solution_current,
                        },
                    )]),
                    password_current,
                )
                .await
                {
                    Ok(msg) => {
                        message.set(Some(msg));
                        puzzle_id.set(String::new());
                        puzzle_solution.set(String::new());
                        puzzle_value.set(String::new());
                        // password.set(String::new()); NOTE should remember password?
                    }
                    Err(e) => {
                        message.set(Some(format!("Error: {}", e)));
                    }
                }
                return;
            }

            match crate::backend::submit_solution(
                username_current.clone(),
                puzzle_current,
                solution_current,
            )
            .await
            {
                Ok(msg) => {
                    message.set(Some(msg));
                    puzzle_id.set(String::new());
                    puzzle_solution.set(String::new());
                    puzzle_value.set(String::new());
                    // password.set(String::new()); NOTE should remember password?
                }
                Err(e) => {
                    message.set(Some(format!("Error: {}", e)));
                }
            }
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "container",
            // TODO get from envpoint
            // h1 { class: "mb-4 font-bold text-lg", "EVENT TITLE PLACEHOLDER" }
            h1 { class: "mb-4 font-bold text-lg",
                if let Some(t) = &*title.read() {
                    "{t}",
                } else {
                    "Betöltés..."
                }
            }

            // Input section
            div { class: "input-section",
                if !*joined.read() {
                    // Join form
                    input { class: INPUT,
                        r#type: "text",
                        placeholder: "Username",
                        value: "{username}",
                        oninput: move |evt| username.set(evt.value())
                    }

                    if *show_password_prompt.read() {
                        input { class: "ml-4 {INPUT}",
                            r#type: "password",
                            placeholder: "Admin Password",
                            value: "{password}",
                            oninput: move |evt| password.set(evt.value())
                        }
                    }

                    button { class: BUTTON, onclick: handle_action, "Join" }
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
                        placeholder: "Solution",
                        value: "{puzzle_solution}",
                        oninput: move |evt| puzzle_solution.set(evt.value())
                    }

                    if *is_admin.read() {
                        input { class: "ml-4 {INPUT}",
                            r#type: "text",
                            placeholder: "Puzlle Value",
                            value: "{puzzle_value}",
                            oninput: move |evt| puzzle_value.set(evt.value())
                        }
                    }

                    if *is_admin.read() {
                        input { class: "ml-4 {INPUT}",
                            r#type: "password",
                            placeholder: "Admin Password",
                            value: "{password}",
                            oninput: move |evt| password.set(evt.value())
                        }
                    }

                    button { class: BUTTON, onclick: handle_action, "Send" }
                }
            }

            // Message display
            // if !message.read().is_empty() {
            //     div { class: "message", "{message}" }
            // }
            if let Some(msg) = &*message.read() {
                div {
                    class: "popup",
                    "{msg}"
                }
            }

            // Teams and puzzles table
            div { class: "table-container",
                table { class: "mt-5",
                    thead {
                        tr {
                            th { class: "text-left pl-2", "." }
                            for (id, value) in puzzles.read().iter() {
                                th {
                                    Tooltip {
                                        TooltipTrigger { "Puzzle {id}" }
                                        TooltipContent {
                                            side: ContentSide::Top,
                                            align: ContentAlign::Center,
                                            div { class: "p-2 border border-(--dark2) rounded-md bg-(--dark)",
                                                "value: {value}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    tbody {
                        for (team_name, solved) in teams_state.read().iter() {
                            tr {
                                td { class: "text-left pl-2 bg-(--dark2)", "{team_name}" }
                                for (puzzle_id, _puzzle) in puzzles.read().iter() {
                                    td { class: "bg-(--dark) text-center",
                                        if solved.contains(puzzle_id) {
                                            "X"
                                        } else {
                                            ""
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
