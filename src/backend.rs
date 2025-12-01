use dioxus::prelude::*;
use std::{
    env,
    sync::{Arc, LazyLock, RwLock},
};

pub use models::*;
mod models;

static EXERCISES: LazyLock<Arc<RwLock<ExerciseState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(ExerciseState::new())));

static TEAMS: LazyLock<Arc<RwLock<TeamsState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(TeamsState::new())));

#[post("/api/join?username")]
pub async fn join(username: String, password: Option<String>) -> Result<String, ServerFnError> {
    let teams = &mut TEAMS.write().unwrap();
    (!teams.contains_key(&username)).or_forbidden("already joined")?;
    // trying to join as admin
    if let Ok(admin_username) = env::var("APOLLO_MESTER_NEV")
        && admin_username == username
    {
        password
            .as_ref()
            .or_unauthorized("password is required for APOLLO_MESTER")?;
        env::var("APOLLO_MESTER_JELSZO")
            .as_ref()
            .is_ok_and(|apw| apw == &password.unwrap())
            .or_unauthorized("incorrect or missing password for APOLLO_MESTER")?; // SAFETY: is_some, see above
    }
    teams.insert(username, SolvedState::new()).unwrap();
    Ok(String::from("helo"))
}

/// Echo the user input on the server.
#[post("/api/{username}?part&solution")]
pub async fn submit_solution(
    username: String,
    part: usize,
    solution: i32,
    password: Option<String>,
) -> Result<String, ServerFnError> {
    Ok(String::from("helo"))
}
