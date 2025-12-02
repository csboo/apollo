use dioxus::fullstack::serde;
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(crate = "dioxus::fullstack::serde")]
pub struct Puzzle {
    pub solution: PuzzleSolution,
    /// how much it's worth
    pub value: PuzzleValue,
}
impl Puzzle {
    pub fn new(solution: PuzzleSolution, value: u32) -> Self {
        Self { solution, value }
    }
}

pub type PuzzleId = String;
pub type PuzzleValue = u32;
pub type PuzzleSolution = String;
pub type PuzzlesExisting = HashMap<PuzzleId, PuzzleValue>;
pub(super) type PuzzleSolutions = HashMap<PuzzleId, Puzzle>;
/// solved puzzles of a team, or existing puzzles in general
pub type SolvedPuzzles = BTreeSet<PuzzleId>;
pub type TeamsState = HashMap<String, SolvedPuzzles>;
