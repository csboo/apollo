use std::time::Duration;

use csv::ReaderBuilder;
use dioxus_primitives::toast::{ToastOptions, Toasts};

use crate::backend::models::{Puzzle, PuzzleId, PuzzleSolutions, PuzzleValue, SolvedPuzzles};

pub fn parse_puzzle_csv(csv_text: &str, toast_api: Toasts) -> PuzzleSolutions {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut puzzles = PuzzleSolutions::new();
    let mut volte = false;

    for result in rdr.records() {
        let record = match result {
            Ok(r) => r,
            Err(_e) => {
                // warn!("skipping invalid CSV row: {}", e);
                volte = true;
                continue;
            }
        };
        let Some(id) = record.get(0) else {
            // warn!("invalid 'id' field in CSV row: {:?}", &record); // TODO dont log value ever
            volte = true;
            continue;
        };
        let Some(solution) = record.get(1) else {
            // warn!("invalid 'solution' field in CSV row: {:?}", &record);
            volte = true;
            continue;
        };
        let Some(value) = record.get(2) else {
            // warn!("invalid 'value' field in CSV row: {:?}", &record);
            volte = true;
            continue;
        };
        let Ok(value_num) = value.parse::<u32>() else {
            // warn!(
            //     "value of field 'value' is not a number in CSV row: {:?}",
            //     &record
            // );
            volte = true;
            continue;
        };

        puzzles.insert(
            id.into(),
            Puzzle {
                solution: solution.into(),
                value: value_num,
            },
        );
    }

    if volte {
        popup_error(
            toast_api,
            "néhány sort nem sikerült betölteni, nézd meg a konzolt",
        );
    }

    puzzles
}

pub fn popup_error(toast_api: Toasts, text: impl std::fmt::Display) {
    toast_api.error(
        "".to_string(),
        ToastOptions::new()
            .description(text)
            .duration(Duration::from_secs(3))
            .permanent(false),
    );
}

pub fn popup_normal(toast_api: Toasts, text: impl std::fmt::Display) {
    toast_api.info(
        "".to_string(),
        ToastOptions::new()
            .description(text)
            .duration(Duration::from_secs(3))
            .permanent(false),
    );
}

pub fn get_points_of(team: &(String, SolvedPuzzles), puzzles: Vec<(PuzzleId, PuzzleValue)>) -> u32 {
    puzzles
        .iter()
        .filter(|(id, _)| team.1.contains(id))
        .map(|(_, value)| *value)
        .sum()
}

pub fn validate_puzzle_id(puzzle_id: &str, toast_api: Toasts) -> bool {
    match !puzzle_id.is_empty() {
        true => true,
        false => {
            popup_error(toast_api, "a feladat nem lehet üres");
            false
        }
    }
}
pub fn validate_puzzle_solution(puzzle_solution: &str, toast_api: Toasts) -> bool {
    match !puzzle_solution.is_empty() {
        true => true,
        false => {
            popup_error(toast_api, "a megoldás nem lehet üres");
            false
        }
    }
}
pub fn validate_puzzle_value(puzzle_value: &str, toast_api: Toasts) -> bool {
    match !puzzle_value.is_empty() {
        true => true,
        false => {
            popup_error(toast_api, "az érték nem lehet üres");
            false
        }
    }
}
