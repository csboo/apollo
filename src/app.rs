use dioxus::fullstack::serde;
use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};

use crate::{
    backend::models::{Puzzle, PuzzleSolutions, PuzzlesExisting, TeamsState},
    components::tooltip::*,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const BUTTON: &str = "ml-4 w-30 px-3 py-2 rounded-lg border border-(--dark2) bg-(--middle) text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const INPUT: &str = "w-50 px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";
const CSV_INPUT: &str = "w-70 px-3 py-2 rounded-lg border border-gray-300 bg-gray-100 text-gray-900 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition";

// TODO could be handeled in much better ways
async fn check_admin_username(username: String) -> Result<bool, ServerFnError> {
    // use std::env;
    let admin_username = "jani";
    Ok(username == admin_username)
}

#[derive(Debug, serde::Deserialize)]
#[serde(crate = "dioxus::fullstack::serde")]
struct PuzzleCsvRow {
    id: String,
    solution: String,
    value: u32,
}

use csv::ReaderBuilder;
fn parse_puzzle_csv(csv_text: &str) -> PuzzleSolutions {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut puzzles = PuzzleSolutions::new();

    for result in rdr.deserialize::<PuzzleCsvRow>() {
        let row = result.expect("invalid csv row");

        puzzles.insert(
            row.id,
            Puzzle {
                solution: row.solution,
                value: row.value,
            },
        );
    }

    puzzles
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
    let mut is_fullscreen = use_signal(|| false);
    let mut parsed_puzzles = use_signal(|| PuzzleSolutions::new());
    trace!("variables inited");

    use_future(move || async move {
        title.set(
            crate::backend::endpoints::event_title()
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
        let mut stream = crate::backend::endpoints::state_stream().await?;
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
                        message.set(Some("Adja meg az admin jelszót".to_string()));
                        return;
                    }
                    joined.set(true);
                    return;
                }
            };

            match crate::backend::endpoints::join(username_current.clone()).await {
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
                match crate::backend::endpoints::set_solution(
                    if parsed_puzzles.read().is_empty() {
                        let value_current_num = value_current.parse::<u32>().unwrap();
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

            match crate::backend::endpoints::submit_solution(
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

        div { class: if *is_fullscreen.read() { "table-only" } else { "normal" },
            div { class: "others-container",
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
                            placeholder: "Csapatnév",
                            value: "{username}",
                            oninput: move |evt| username.set(evt.value())
                        }

                        if *show_password_prompt.read() {
                            input { class: "ml-4 {INPUT}",
                                r#type: "password",
                                placeholder: "Admin jelszó",
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value())
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

                        if *is_admin.read() {
                            input { class: "ml-4 {INPUT}",
                                r#type: "text",
                                placeholder: "Érték/Nyeremény",
                                value: "{puzzle_value}",
                                oninput: move |evt| puzzle_value.set(evt.value())
                            }

                            input { class: "ml-4 {INPUT}",
                                r#type: "password",
                                placeholder: "Admin jelszó",
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value())
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
                if let Some(msg) = &*message.read() {
                    div {
                        class: "popup",
                        "{msg}"
                    }
                }
            }
            // Teams and puzzles table
            div { class: "table-container",
                table { class: "mt-5",
                    onclick: toggle_fullscreen,
                    thead {
                        tr {
                            th { class: "text-left pl-2", "." }
                            for (id, value) in puzzles.read().iter() {
                                th {
                                    Tooltip {
                                        TooltipTrigger { class: "text-(--light)", "Puzzle {id}" }
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
                                td { class: "text-left pl-2 text-(--light) bg-(--dark2)", "{team_name}" }
                                for (puzzle_id, _puzzle) in puzzles.read().iter() {
                                    td { class: "text-(--light) bg-(--dark) text-center text-[30px] font-[900]",
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
