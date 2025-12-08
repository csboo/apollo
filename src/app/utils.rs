use csv::ReaderBuilder;
use dioxus::prelude::*;

use crate::backend::models::{Puzzle, PuzzleSolutions};

pub fn parse_puzzle_csv(csv_text: &str) -> PuzzleSolutions {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut puzzles = PuzzleSolutions::new();

    for result in rdr.records() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                warn!("Skipping invalid CSV row: {}", e);
                continue;
            }
        };
        let id = record.get(0).unwrap().to_string();
        let solution = record.get(1).unwrap().to_string();
        let value: u32 = record.get(2).unwrap().parse().unwrap();

        puzzles.insert(id, Puzzle { solution, value });
    }

    puzzles
}
