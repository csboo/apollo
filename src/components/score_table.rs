use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};

use crate::{
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
    components::tooltip::*,
};

#[component]
pub fn ScoreTable(
    puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
    teams_state: Signal<Vec<(PuzzleId, SolvedPuzzles)>>,
    toggle_fullscreen: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
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
