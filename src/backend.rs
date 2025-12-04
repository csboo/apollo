use dioxus::fullstack::{CborEncoding, Streaming};
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
}

static ADMIN_PASSWORD: LazyLock<String> = LazyLock::new(|| ensure_env_var("APOLLO_MESTER_JELSZO"));
static EVENT_TITLE: LazyLock<Result<String, env::VarError>> =
    LazyLock::new(|| env::var("APOLLO_EVENT_TITLE"));

#[get("/api/event_title")]
pub async fn event_title() -> Result<String> {
    Ok(EVENT_TITLE.clone()?)
}

async fn get_game_state() -> (TeamsState, PuzzlesExisting) {
    let existing_puzzles = PUZZLES
        .read()
        .unwrap()
        .clone()
        .into_iter()
        .map(|(id, sol)| (id, sol.value))
        .collect();
    (TEAMS.read().unwrap().clone(), existing_puzzles)
}

/// just save a copy of the `PUZZLES` and `TEAMS` state to disk into a `cbor` file
/// TODO: add basic encryption using `ADMIN_PASSWORD`
#[server]
async fn backup_state() -> Result<()> {
    let teams_state = TEAMS.read().unwrap().clone();
    let puzzles_state = PUZZLES.read().unwrap().clone();
    let mut buf = vec![];
    ciborium::into_writer(&(teams_state, puzzles_state), &mut buf)
        .inspect_err(|e| error!("couldn't serialize into cbor: {e}"))?;
    tokio::fs::write("apollo-state.cbor", buf)
        .await
        .inspect_err(|e| error!("couldn't write state to file: {e}"))?;
    Ok(())
}

/// streams current progress of the teams and existing puzzles with their values
#[get("/api/state")]
pub async fn state_stream() -> Result<Streaming<(TeamsState, PuzzlesExisting), CborEncoding>> {
    Ok(Streaming::spawn(|tx| async move {
        while tx.unbounded_send(get_game_state().await).is_ok() {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }))
}

/// join the competition as a contestant team
#[post("/api/join")]
pub async fn join(username: String) -> Result<String, HttpError> {
    _ = backup_state().await;
    let teams = &mut TEAMS.write().unwrap();
    (!teams.contains_key(&username)).or_forbidden("taken username")?;
    _ = teams.insert(username, SolvedPuzzles::new());
    Ok(String::from("helo, mehet!"))
}

/// set `puzzle_id`'s a `solution` and `value` with `ADMIN_PASSWORD`
#[post("/api/set_solution")]
pub async fn set_solution(
    puzzle_solutions: PuzzleSolutions,
    password: String,
) -> Result<String, HttpError> {
    _ = backup_state().await;
    // submitting as admin
    (*ADMIN_PASSWORD == password).or_unauthorized("incorrect password for APOLLO_MESTER")?;

    let puzzles_state = &mut PUZZLES.write().unwrap();
    puzzle_solutions
        .keys()
        .any(|new_k| !puzzles_state.contains_key(new_k))
        .or_forbidden("one of the puzzles already set")?;

    puzzles_state.extend(puzzle_solutions);

    Ok(String::from("beallitottam a megoldast"))
}

/// submit a solution as a team
#[post("/api/submit")]
pub async fn submit_solution(
    username: String,
    puzzle_id: PuzzleId,
    solution: PuzzleSolution,
) -> Result<String, HttpError> {
    _ = backup_state().await;
    let teams = &mut TEAMS.write().unwrap();
    let team_state = teams
        .get_mut(&username)
        .or_forbidden("no such team in the competition, join first")?;

    let puzzles = &mut PUZZLES.read().unwrap();
    if solution
        == *puzzles
            .get(&puzzle_id)
            .or_not_found("no such puzzle")?
            .solution
    {
        team_state
            .insert(puzzle_id)
            .or_forbidden("already solved this puzzle")?;
        Ok(String::from("oke, megoldottad, elmentettem!"))
    } else {
        HttpError::forbidden("incorrect solution")?
    }
}
