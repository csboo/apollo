use dioxus::fullstack::serde;
use std::collections::{HashMap, HashSet};

// SECURITY: SecretString, with manual impls?
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(crate = "dioxus::fullstack::serde")]
pub struct Puzzle {
    pub solution: PuzzleSolution,
    /// how much it's worth
    pub value: PuzzleValue,
}

pub type PuzzleId = String;
/// how much points you get for solving a puzzle
pub type PuzzleValue = u32;
pub type PuzzleSolution = String;
/// all the known puzzles with their values
pub type PuzzlesExisting = HashMap<PuzzleId, PuzzleValue>;
/// all the puzzles with their values and solutions
pub type PuzzleSolutions = HashMap<PuzzleId, Puzzle>;
/// solved puzzles of a team
pub type SolvedPuzzles = HashSet<PuzzleId>;
/// progress of each team, which puzzles they've solved
pub type TeamsState = HashMap<String, SolvedPuzzles>;
