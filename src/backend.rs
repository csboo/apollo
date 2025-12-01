use dioxus::prelude::*;
use std::{
    env, process,
    sync::{Arc, LazyLock, RwLock},
};

pub use models::*;
mod models;

static EXERCISES: LazyLock<Arc<RwLock<ExerciseState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(ExerciseState::new())));

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
    if *ADMIN_USERNAME == username {
        password
            .as_ref()
            .or_bad_request("password is required for APOLLO_MESTER")?;

        // SAFETY: is_some, see above
        if *ADMIN_PASSWORD != password.unwrap() {
            return HttpError::unauthorized("incorrect password for APOLLO_MESTER")?;
        }
    }
    _ = teams.insert(username, SolvedState::new());
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
