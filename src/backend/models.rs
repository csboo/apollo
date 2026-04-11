use dioxus::fullstack::serde;
use std::collections::HashMap;
use std::time::SystemTime;

// SECURITY: SecretString, with manual impls?
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(crate = "dioxus::fullstack::serde")]
pub struct Puzzle {
    /// argon2-encoded solution hash (not the raw solution)
    pub solution: PuzzleSolutionHash,
    /// how much it's worth
    pub value: PuzzleValue,
}

pub type PuzzleId = String;
/// how much points you get for solving a puzzle
pub type PuzzleValue = u32;
pub type PuzzleSolution = String;
pub type PuzzleSolutionHash = String;
/// all the known puzzles with their values
pub type PuzzlesExisting = HashMap<PuzzleId, PuzzleValue>;
/// all the puzzles with their values and solutions
pub type PuzzleSolutions = HashMap<PuzzleId, Puzzle>;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(crate = "dioxus::fullstack::serde")]
pub struct SolveAttempt {
    pub puzzle_id: PuzzleId,
    pub attempted_at: SystemTime,
    pub correct: bool,
}
impl SolveAttempt {
    pub fn now(puzzle_id: PuzzleId, correct: bool) -> Self {
        Self {
            puzzle_id,
            attempted_at: SystemTime::now(),
            correct,
        }
    }
}

/// all solve attempts of a team
pub type TeamAttempts = Vec<SolveAttempt>;
/// progress of each team, which puzzles they attempted and when
pub type TeamsState = HashMap<String, TeamAttempts>;

pub fn team_has_solved_puzzle(team_attempts: &[SolveAttempt], puzzle_id: &str) -> bool {
    team_attempts
        .iter()
        .any(|attempt| attempt.puzzle_id == puzzle_id && attempt.correct)
}

#[cfg(test)]
mod tests {
    use super::{SolveAttempt, team_has_solved_puzzle};
    use std::time::UNIX_EPOCH;

    #[test]
    fn marks_puzzle_as_solved_only_for_correct_attempts() {
        let team_attempts = vec![
            SolveAttempt {
                puzzle_id: "p1".into(),
                attempted_at: UNIX_EPOCH,
                correct: false,
            },
            SolveAttempt {
                puzzle_id: "p1".into(),
                attempted_at: UNIX_EPOCH,
                correct: true,
            },
        ];

        assert!(team_has_solved_puzzle(&team_attempts, "p1"));
        assert!(!team_has_solved_puzzle(&team_attempts, "p2"));
    }
}
