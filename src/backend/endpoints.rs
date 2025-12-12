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
/// - got `sid` cookie
///   - valid => forbidden
///   - invalid => goto #no-sid-cookie
/// - no `sid` cookie
///   - `username`'s session is taken => forbidden
///   - otherwise => allowed, preserve progress if any
///
/// We'll return a `SetCookie` header if the login is successful.
///
/// This will set a cookie in the user's browser that can be used for subsequent authenticated requests.
#[post("/api/join", cookies: TypedHeader<Cookie>)]
pub async fn join(username: String) -> Result<SetHeader<SetCookie>, HttpError> {
    if let Ok(sent_uuid) = extract_sid_cookie(cookies).await
        && USER_IDS.read().await.contains_key(&sent_uuid)
    {
        return HttpError::forbidden("already logged in");
    }

    // whether someone's currently logged in to this account: `USER_IDS` contains `username`
    (!USER_IDS.read().await.values().any(|u| u == &username)).or_forbidden("taken session")?;

    // brand new team
    if !TEAMS.read().await.contains_key(&username) {
        _ = TEAMS
            .write()
            .await
            .insert(username.clone(), SolvedPuzzles::new());
    }
    // allowed to log in, but don't reset progress

    let uuid = Uuid::new_v4();
    _ = USER_IDS.write().await.insert(uuid, username);

    #[cfg(feature = "server_state_save")]
    state_save::save_state().await?;

    SetHeader::new(format!("sid={uuid};HttpOnly;Secure;SameSite=Strict"))
        .or_internal_server_error("invalid sid cookie")
}

/// log out of the competition, but preserve progress of the team for future relogins
///
/// returns empty, expired `sid` `SetCookie` header => browser deletes the valid one => user's now deauthed
#[get("/api/logout", cookies: TypedHeader<Cookie>)]
pub async fn logout() -> Result<SetHeader<SetCookie>, HttpError> {
    let uuid = extract_sid_cookie(cookies)
        .await
        .or_bad_request("not logged in, not logging out")?;
    _ = USER_IDS.write().await.remove(&uuid);
    // this makes the client invalidate the actual sid cookie
    SetHeader::new("sid=;Expires=Thu, 01 Jan 1970 00:00:00 GMT")
        .or_internal_server_error("invalid deauth sid cookie")
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

    PUZZLES.write().await.extend(puzzle_solutions);

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
