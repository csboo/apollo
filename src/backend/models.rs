use std::collections::HashMap;

pub type ExerciseState = HashMap<usize, i32>;
pub type SolvedState = HashMap<usize, bool>;
pub type TeamsState = HashMap<String, SolvedState>;
