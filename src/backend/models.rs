use std::collections::{BTreeSet, HashMap};

pub type PuzzleSolution = i32;
pub type PuzzleId = usize;
pub type PuzzleSolutions = HashMap<PuzzleId, PuzzleSolution>;
/// solved puzzles of a team, or existing puzzles in general
pub type SolvedPuzzles = BTreeSet<PuzzleId>;
pub type TeamsState = HashMap<String, SolvedPuzzles>;
