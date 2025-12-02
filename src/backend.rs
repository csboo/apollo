use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use std::sync::{LazyLock, RwLock};
use std::{env, process};

pub use models::*;
mod models;

static PUZZLES: LazyLock<RwLock<PuzzleSolutions>> =
    LazyLock::new(|| RwLock::new(PuzzleSolutions::new()));

static TEAMS: LazyLock<RwLock<TeamsState>> = LazyLock::new(|| RwLock::new(TeamsState::new()));

/// without `name`, the app won't run
fn ensure_env_var(key: &str) -> String {
    let Ok(value) = env::var(key) else {
        error!("{key:?} env var not set, can't proceed");
        process::exit(1);
    };
    if value.is_empty() {
        error!("{key:?} env var empty, can't proceed");
        process::exit(1);
    }
    value
}

/// # exits with 1
/// if necessary admin env vars aren't set
pub fn ensure_admin_env_vars() {
    _ = LazyLock::force(&ADMIN_PASSWORD);
    _ = LazyLock::force(&ADMIN_USERNAME);
}

pub static ADMIN_USERNAME: LazyLock<String> = LazyLock::new(|| ensure_env_var("APOLLO_MESTER_NEV"));
static ADMIN_PASSWORD: LazyLock<String> = LazyLock::new(|| ensure_env_var("APOLLO_MESTER_JELSZO"));

/// streams current progress of the teams and existing puzzles with their values
#[get("/api/state_json_stream")]
pub async fn state_stream() -> Result<Streaming<(TeamsState, PuzzleSolutions), JsonEncoding>> {
    Ok(Streaming::spawn(|tx| async move {
        while tx
            .unbounded_send((
                TEAMS.read().unwrap().clone(),
                PUZZLES.read().unwrap().clone(),
            ))
            .is_ok()
        {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }))
}

/// join the competition as a contestant team
#[post("/api/join")]
pub async fn join(username: String) -> Result<String, HttpError> {
    let teams = &mut TEAMS.write().unwrap();
    (username != *ADMIN_USERNAME && !teams.contains_key(&username))
        .or_forbidden("taken username")?;
    _ = teams.insert(username, SolvedPuzzles::new());
    Ok(String::from("helo, mehet!"))
}

/// submit a solution either as a team, or as `ADMIN_USERNAME` with a `password`
#[post("/api/submit")]
pub async fn submit_solution(
    username: String,
    puzzle_id: PuzzleId,
    solution: PuzzleSolution,
    // only needed if submitting (setting) as `ADMIN`
    value: Option<PuzzleValue>,
    password: Option<String>,
) -> Result<String, HttpError> {
    // submitting as admin
    if *ADMIN_USERNAME == username {
        if *ADMIN_PASSWORD != password.or_bad_request("password is required for APOLLO_MESTER")? {
            return HttpError::unauthorized("incorrect password for APOLLO_MESTER")?;
        }

        let puzzles = &mut PUZZLES.write().unwrap();
        (!puzzles.contains_key(&puzzle_id))
            .or_forbidden("a solution for this puzzle is already set")?;
        let set_puzzle = Puzzle::new(solution, value.or_bad_request("missing solution")?);
        puzzles.insert(puzzle_id, set_puzzle);
        return Ok("beallitottam a megoldast".to_string());
    }

    let teams = &mut TEAMS.write().unwrap();
    let team_state = teams
        .get_mut(&username)
        .or_forbidden("no such team in the competition, join first")?;

    let puzzles = &mut PUZZLES.read().unwrap();
    if solution
        == puzzles
            .get(&puzzle_id)
            .or_not_found("no such puzzle")?
            .solution()
    {
        team_state
            .insert(puzzle_id)
            .or_forbidden("already solved this puzzle")?;
        Ok(String::from("oke, megoldottad, elmentettem!"))
    } else {
        HttpError::forbidden("incorrect solution")?
    }
}
