use super::models::*;
use dioxus::fullstack::{CborEncoding, SetCookie, SetHeader, Streaming};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use {
    super::logic::*,
    dioxus::fullstack::{Cookie, TypedHeader},
    secrecy::ExposeSecret,
    uuid::Uuid,
};

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

/// returns username if valid
#[get("/api/auth_state", cookies: TypedHeader<Cookie>)]
pub async fn auth_state() -> Result<String, HttpError> {
    let uuid = extract_sid_cookie(cookies).await?;
    let username = USER_IDS
        .read()
        .await
        .get(&uuid)
        .or_unauthorized("no such userid")?
        .clone();
    Ok(username)
}

/// join the competition as a contestant team
///
/// We'll return a `SetCookie` header if the login is successful.
///
/// This will set a cookie in the user's browser that can be used for subsequent authenticated requests.
#[post("/api/join")]
pub async fn join(username: String) -> Result<SetHeader<SetCookie>, HttpError> {
    (!TEAMS.read().await.contains_key(&username)).or_forbidden("taken username")?;

    let mut teams_lock = TEAMS.write().await;
    _ = teams_lock.insert(username.clone(), SolvedPuzzles::new());
    drop(teams_lock);

    let uuid = Uuid::new_v4();
    _ = USER_IDS.write().await.insert(uuid, username);

    #[cfg(feature = "server_state_save")]
    state_save::save_state().await?;

    SetHeader::new(format!("sid={uuid};HttpOnly;Secure;SameSite=Strict"))
        .or_internal_server_error("invalid sid cookie")
}

/// set `puzzle_solutions` with `ADMIN_PASSWORD`
///
/// NOTE: if any of the solutions is incorrect, none will be saved
#[post("/api/set_solution")]
pub async fn set_solution(
    puzzle_solutions: PuzzleSolutions,
    password: String,
) -> Result<String, HttpError> {
    // submitting as admin
    (*ADMIN_PASSWORD.expose_secret() == password)
        .or_unauthorized("incorrect password for APOLLO_MESTER")?;

    let puzzles_lock = PUZZLES.read().await;
    puzzle_solutions
        .keys()
        .any(|new_k| !puzzles_lock.contains_key(new_k))
        .or_forbidden("one of the puzzles already set")?;
    drop(puzzles_lock);

    let mut puzzles_lock = PUZZLES.write().await;
    puzzles_lock.extend(puzzle_solutions);
    drop(puzzles_lock);

    #[cfg(feature = "server_state_save")]
    state_save::save_state().await?;

    Ok(String::from("beallitottam a megoldast"))
}

/// submit a solution as a team
///
/// We'll use the `TypedHeader` extractor on the server to get the cookie from the request.
#[post("/api/submit", cookies: TypedHeader<Cookie>)]
pub async fn submit_solution(
    puzzle_id: PuzzleId,
    solution: PuzzleSolution,
) -> Result<String, HttpError> {
    let uuid = extract_sid_cookie(cookies).await?;
    let username = USER_IDS
        .read()
        .await
        .get(&uuid)
        .or_unauthorized("no such userid")?
        .clone(); // PERF: rather clone than lock

    PUZZLES
        .read()
        .await
        .get(&puzzle_id)
        .or_not_found("no such puzzle")?
        .solution
        .eq(&solution)
        .or_forbidden("incorrect solution")?;

    let mut teams_lock = TEAMS.write().await;

    let team_solved = teams_lock
        .get_mut(&username)
        .or_internal_server_error("shouldn't have got this far")?;

    team_solved
        .insert(puzzle_id)
        .or_forbidden("already solved this puzzle")?;
    drop(teams_lock);

    #[cfg(feature = "server_state_save")]
    state_save::save_state().await?;

    Ok(String::from("oke, megoldottad, elmentettem!"))
}
