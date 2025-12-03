use dioxus::prelude::*;

use crate::{
    backend::{PuzzlesExisting, TeamsState},
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const BUTTON: &str = "ml-4 w-30 px-3 py-2 rounded-lg border border-(--dark2) bg-(--middle) text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const INPUT: &str = "w-50 px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";

// Server function to check if username is admin (only one we need to add)
async fn check_admin_username(username: String) -> Result<bool, ServerFnError> {
    // use std::env;
    let admin_username = "jani";
    Ok(username == admin_username)
}

#[component]
pub fn App() -> Element {
    // State management
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut puzzle_id = use_signal(|| String::new());
    let mut puzzle_solution = use_signal(|| String::new());
    let mut joined = use_signal(|| false);
    let mut is_admin = use_signal(|| false);
    let mut show_password_prompt = use_signal(|| false);
    let mut teams_state = use_signal(|| TeamsState::new());
    let mut puzzles = use_signal(|| PuzzlesExisting::new());
    let mut message = use_signal(|| String::new());

    use_future(move || async move {
        // Call the SSE endpoint to get a stream of events
        let mut stream = crate::backend::state_stream().await?;

        // And then poll it for new events, adding them to our signal
        while let Some(Ok(data)) = stream.next().await {
            teams_state.set(data.0);
            puzzles.set(data.1);
        }

        dioxus::Ok(())
    });

    // Handle join/submit button click
    let handle_action = move |_| {
        spawn(async move {
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
                            message.set("Please enter admin password".to_string());
                            return;
                        }
                        joined.set(true);
                        return;
                    }
                }

                // Join team - call backend function directly
                let pwd = if admin || *show_password_prompt.read() {
                    Some(password_current.clone())
                } else {
                    None
                };

                match crate::backend::join(username_current.clone()).await {
                    Ok(msg) => {
                        message.set(msg);
                        joined.set(true);
                        password.set(String::new());
                        show_password_prompt.set(false);
                    }
                    Err(e) => {
                        message.set(format!("Error: {}", e));
                    }
                }
            } else {
                // Submit solution - call backend function directly
                let puzzle_current = puzzle_id.read().clone();
                let solution_current = puzzle_solution.read().clone();
                // let value_current = puzzle_value_FROMUI.read().clone();

                let pwd = if admin {
                    Some(password_current.clone())
                } else {
                    None
                };

                match crate::backend::submit_solution(
                    username_current.clone(),
                    puzzle_current,
                    solution_current,
                    None,
                    pwd,
                )
                .await
                {
                    Ok(msg) => {
                        message.set(msg);
                        puzzle_id.set(String::new());
                        puzzle_solution.set(String::new());
                        password.set(String::new());
                    }
                    Err(e) => {
                        message.set(format!("Error: {}", e));
                    }
                }
            }
        });
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "container",
            h1 { "Apollo Hackathon Tracker" }

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
            if !message.read().is_empty() {
                div { class: "message", "{message}" }
            }

            // Teams and puzzles table
            div { class: "table-container",
                table { class: "mt-5",
                    thead {
                        tr {
                            th { class: "text-left pl-2", "." }
                            for (id, value) in puzzles.read().iter() {
                                th { "Puzzle {id}" }
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
