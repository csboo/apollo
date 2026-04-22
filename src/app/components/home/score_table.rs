use dioxus::prelude::*;

use crate::backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles};

#[component]
pub fn ScoreTable(
    puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
    teams_state: Signal<Vec<(PuzzleId, SolvedPuzzles)>>,
    is_fullscreen: bool,
    toggle_fullscreen: EventHandler<MouseEvent>,
) -> Element {
    let puzzle_limit = if is_fullscreen { usize::MAX } else { 3 };
    let team_limit = if is_fullscreen { usize::MAX } else { 3 };
    let has_data = !puzzles.read().is_empty() || !teams_state.read().is_empty();

    rsx! {
        div {
            class: format!("score-table-shell {}", if is_fullscreen { "w-full" } else { "score-table-shell-preview" }),
            onclick: toggle_fullscreen,
            cursor: "pointer",

            if has_data {
                table { class: "score-table w-full",
                    thead {
                        tr {
                            th { class: "text-left h-14 px-4 bg-(--bg-elevated) text-(--text-secondary) text-sm font-medium",
                                "Csapat"
                            }
                            for (id, value) in puzzles.read().iter().take(puzzle_limit) {
                                th { class: "h-14 px-4 bg-(--bg-elevated) text-center",
                                    div { class: "text-sm font-medium text-(--text-primary)",
                                        "{id}"
                                    }
                                    div { class: "text-xs text-(--text-muted) mt-0.5",
                                        "{value} pont"
                                    }
                                }
                            }
                        }
                    }
                    tbody {
                        for (team_name, solved) in teams_state.read().iter().take(team_limit) {
                            tr { class: "group",
                                td { class: "text-left px-4 py-3 font-medium text-(--text-primary) bg-(--bg-elevated) group-hover:bg-(--bg-surface) transition-colors",
                                    "{team_name}"
                                }
                                for (puzzle_id, _puzzle) in puzzles.read().iter().take(puzzle_limit) {
                                    td { class: "text-center py-3 bg-(--bg) group-hover:bg-(--bg-elevated) transition-colors",
                                        if solved.contains(puzzle_id) {
                                            span { class: "inline-flex items-center justify-center w-7 h-7 rounded-full bg-(--accent-success)/15 text-(--accent-success)",
                                                "✓"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Empty state
                div { class: "flex flex-col items-center justify-center py-16 px-6 text-center",
                    div { class: "w-14 h-14 mb-4 rounded-full bg-(--bg-surface) flex items-center justify-center text-2xl",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",
                            class: "size-6",

                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M10.5 6a7.5 7.5 0 1 0 7.5 7.5h-7.5V6Z"
                            }
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M13.5 10.5H21A7.5 7.5 0 0 0 13.5 3v7.5Z"
                            }
                        }
                    }
                    p { class: "text-(--text-muted) text-sm",
                        "Még nincsenek adatok"
                    }
                }
            }

            // Preview overlay
            if !is_fullscreen && has_data {
                div { class: "score-table-preview-overlay",
                    span { class: "score-table-preview-overlay-text",
                        "Kattints a teljes táblához"
                    }
                }
            }
        }
    }
}
