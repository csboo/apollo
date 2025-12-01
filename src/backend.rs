use dioxus::prelude::*;
use std::{
    env, process,
    sync::{Arc, LazyLock, RwLock},
};

pub use models::*;
mod models;

static PUZZLES: LazyLock<Arc<RwLock<PuzzleSolutions>>> =
    LazyLock::new(|| Arc::new(RwLock::new(PuzzleSolutions::new())));

static TEAMS: LazyLock<Arc<RwLock<TeamsState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(TeamsState::new())));

static ADMIN_USERNAME: LazyLock<String> = LazyLock::new(|| {
    const ADMIN_USERNAME_ENV_VAR_NAME: &str = "APOLLO_MESTER_NEV";
    let Ok(admin_username) = env::var(ADMIN_USERNAME_ENV_VAR_NAME) else {
        error!("{ADMIN_USERNAME_ENV_VAR_NAME} env var not set, can't proceed");
        process::exit(1);
    };
    admin_username
});

static ADMIN_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    const ADMIN_PASSWORD_ENV_VAR_NAME: &str = "APOLLO_MESTER_JELSZO";
    let Ok(admin_password) = env::var(ADMIN_PASSWORD_ENV_VAR_NAME) else {
        error!("{ADMIN_PASSWORD_ENV_VAR_NAME} env var not set, can't proceed");
        process::exit(2);
    };
    admin_password
});

#[post("/api/join?username")]
pub async fn join(username: String, password: Option<String>) -> Result<String, HttpError> {
    let teams = &mut TEAMS.write().unwrap();
    (!teams.contains_key(&username)).or_forbidden("already joined")?;
    // trying to join as admin
    if *ADMIN_USERNAME == username
        && *ADMIN_PASSWORD != password.or_bad_request("password is required for APOLLO_MESTER")?
    {
        return HttpError::unauthorized("incorrect password for APOLLO_MESTER")?;
    }
    _ = teams.insert(username, SolvedPuzzles::new());
    Ok(String::from("helo, mehet!"))
}

/// Echo the user input on the server.
#[post("/api/submit?username&puzzle&solution")]
pub async fn submit_solution(
    username: String,
    puzzle: usize,
    solution: i32,
    password: Option<String>,
) -> Result<String, HttpError> {
    let teams = &mut TEAMS.write().unwrap();
    let team_state = teams
        .get_mut(&username)
        .or_forbidden("no such team in the competition, join first")?;

    // submitting as admin
    if *ADMIN_USERNAME == username {
        if *ADMIN_PASSWORD != password.or_bad_request("password is required for APOLLO_MESTER")? {
            return HttpError::unauthorized("incorrect password for APOLLO_MESTER")?;
        }

        let puzzles = &mut PUZZLES.write().unwrap();
        (!puzzles.contains_key(&puzzle))
            .or_forbidden("a solution for this puzzle is already set")?;
        puzzles.insert(puzzle, solution);
    }

    let puzzles = &mut PUZZLES.read().unwrap();
    if solution == *puzzles.get(&puzzle).or_not_found("no such puzzle")? {
        team_state
            .insert(puzzle)
            .or_forbidden("already solved this puzzle")?;
        Ok(String::from("oke, megoldottad, elmentettem!"))
    } else {
        HttpError::forbidden("incorrect solution")?
    }
}
