use super::models::*;
use dioxus::prelude::*;
use std::sync::{LazyLock, RwLock};
use std::{env, process};

pub(super) static PUZZLES: LazyLock<RwLock<PuzzleSolutions>> =
    LazyLock::new(|| RwLock::new(PuzzleSolutions::new()));

pub(super) static TEAMS: LazyLock<RwLock<TeamsState>> =
    LazyLock::new(|| RwLock::new(TeamsState::new()));

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

pub(super) static ADMIN_PASSWORD: LazyLock<String> =
    LazyLock::new(|| ensure_env_var("APOLLO_MESTER_JELSZO"));
pub(super) static EVENT_TITLE: LazyLock<Result<String, env::VarError>> =
    LazyLock::new(|| env::var("APOLLO_EVENT_TITLE"));

pub(super) async fn get_game_state() -> (TeamsState, PuzzlesExisting) {
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
pub(super) async fn backup_state() -> Result<()> {
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
