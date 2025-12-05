#[cfg(feature = "server")]
use super::logic::*;
use super::models::*;
use dioxus::fullstack::{CborEncoding, Streaming};
use dioxus::prelude::*;

#[get("/api/event_title")]
pub async fn event_title() -> Result<String> {
    Ok(EVENT_TITLE.clone()?)
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
