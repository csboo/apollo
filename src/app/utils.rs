use csv::ReaderBuilder;
use dioxus::prelude::*;

use crate::backend::models::{Puzzle, PuzzleId, PuzzleSolutions};

pub fn parse_puzzle_csv(csv_text: &str) -> PuzzleSolutions {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut puzzles = PuzzleSolutions::new();

    for result in rdr.deserialize::<(PuzzleId, Puzzle)>() {
        match result {
            Ok((id, puzzle)) => {
                puzzles.insert(id, puzzle);
            }
            Err(e) => {
                warn!("Skipping invalid CSV row: {}", e);
            }
        }
    }

    puzzles
}
