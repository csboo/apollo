use dioxus::fullstack::serde;
use jiff::Timestamp;
use std::collections::HashMap;

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

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(crate = "dioxus::fullstack::serde")]
pub enum SolveAttemptState {
    Correct,
    Incorrect,
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(crate = "dioxus::fullstack::serde")]
pub struct SolveAttempt {
    pub puzzle_id: PuzzleId,
    pub attempted_at: Timestamp,
    pub state: SolveAttemptState,
}

/// all solve attempts of a team
pub type TeamAttempts = Vec<SolveAttempt>;
/// progress of each team, which puzzles they attempted and when
pub type TeamsState = HashMap<String, TeamAttempts>;

pub fn team_has_solved_puzzle(team_attempts: &[SolveAttempt], puzzle_id: &str) -> bool {
    team_attempts.iter().any(|attempt| {
        attempt.puzzle_id == puzzle_id && matches!(attempt.state, SolveAttemptState::Correct)
    })
}

#[cfg(test)]
mod tests {
    use super::{SolveAttempt, SolveAttemptState, team_has_solved_puzzle};
    use jiff::Timestamp;

    #[test]
    fn marks_puzzle_as_solved_only_for_correct_attempts() {
        let team_attempts = vec![
            SolveAttempt {
                puzzle_id: "p1".into(),
                attempted_at: Timestamp::UNIX_EPOCH,
                state: SolveAttemptState::Incorrect,
            },
            SolveAttempt {
                puzzle_id: "p1".into(),
                attempted_at: Timestamp::UNIX_EPOCH,
                state: SolveAttemptState::Correct,
            },
        ];

        assert!(team_has_solved_puzzle(&team_attempts, "p1"));
        assert!(!team_has_solved_puzzle(&team_attempts, "p2"));
    }
}
