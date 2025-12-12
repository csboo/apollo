use csv::ReaderBuilder;
use dioxus::prelude::*;

use crate::{
    app::models::Message,
    backend::models::{Puzzle, PuzzleSolutions},
};

pub fn parse_puzzle_csv(
    csv_text: &str,
    message: Signal<Option<(Message, String)>>,
) -> PuzzleSolutions {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut puzzles = PuzzleSolutions::new();
    let mut volte = false;

    for result in rdr.records() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                warn!("skipping invalid CSV row: {}", e);
                volte = true;
                continue;
            }
        };
        let Some(id) = record.get(0) else {
            warn!("invalid 'id' field in CSV row: {:?}", &record); // TODO dont log value ever 
            volte = true;
            continue;
        };
        let Some(solution) = record.get(1) else {
            warn!("invalid 'solution' field in CSV row: {:?}", &record);
            volte = true;
            continue;
        };
        let Some(value) = record.get(2) else {
            warn!("invalid 'value' field in CSV row: {:?}", &record);
            volte = true;
            continue;
        };
        let Ok(value_num) = value.parse::<u32>() else {
            warn!(
                "value of field 'value' is not a number in CSV row: {:?}",
                &record
            );
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
            message.clone(),
            "néhány sort nem sikerült betölteni, nézd meg a konzolt",
        );
    }

    puzzles
}

pub fn popup_error(
    mut signal_message: Signal<Option<(Message, String)>>,
    text: impl std::fmt::Display,
) {
    signal_message.set(Some((Message::MsgErr, text.to_string())));
}

pub fn popup_normal(
    mut signal_message: Signal<Option<(Message, String)>>,
    text: impl std::fmt::Display,
) {
    signal_message.set(Some((Message::MsgNorm, text.to_string())));
}
