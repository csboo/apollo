use std::collections::{BTreeSet, HashMap};

pub type PuzzleSolutions = HashMap<usize, i32>;
pub type SolvedPuzzles = BTreeSet<usize>;
pub type TeamsState = HashMap<String, SolvedPuzzles>;
