use super::models::*;
use dioxus::fullstack::{CborEncoding, SetCookie, SetHeader, Streaming};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use {
    super::{CookieMap, i18n::Localizer, logic::*},
    crate::{s_t, s_tid},
    dioxus::fullstack::HeaderMap,
    uuid::Uuid,
    zeroize::Zeroize,
};

#[get("/api/event_title")]
pub async fn event_title() -> Result<String> {
    Ok(EVENT_TITLE.clone()?)
}

/// streams current progress of the teams and existing puzzles with their values
#[get("/api/state", headers: HeaderMap)]
pub async fn state_stream() -> Result<Streaming<(TeamsState, PuzzlesExisting), CborEncoding>> {
    check_admin_pwd(&Localizer::from_headers(&headers))?;
    Ok(Streaming::spawn(|tx| async move {
        while tx.unbounded_send(get_game_state().await).is_ok() {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }))
}

/// returns username if valid
#[get("/api/auth_state", headers: HeaderMap, cookies: CookieMap)]
pub async fn auth_state() -> Result<String, HttpError> {
    let i18n = Localizer::from_headers(&headers);
    check_admin_pwd(&i18n)?;
    let uuid = extract_sid_cookie_localized(cookies, &i18n).await?;
    let username = USER_IDS
        .read()
        .await
        .get(&uuid)
        .or_not_found(s_t!(i18n, "uuid-no-team"))?
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
///
/// WARN: **always** returns `Some(Ok(SetHeader { data: None }))`, see <https://github.com/DioxusLabs/dioxus/issues/5089>
#[post("/api/join", headers: HeaderMap, cookies: CookieMap)]
pub async fn join(username: String) -> Result<SetHeader<SetCookie>, HttpError> {
    let i18n = Localizer::from_headers(&headers);
    check_admin_pwd(&i18n)?;
    if let Ok(sent_uuid) = extract_sid_cookie_localized(cookies, &i18n).await
        && USER_IDS.read().await.contains_key(&sent_uuid)
    {
        return HttpError::forbidden(s_t!(i18n, "already-in"));
    }

    // whether someone's currently logged in to this account: `USER_IDS` contains `username`
    (!USER_IDS.read().await.values().any(|u| u == &username))
        .or_forbidden(s_t!(i18n, "taken-session"))?;

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
    tokio::spawn(state_save::save_state());

    SetHeader::new(format!("sid={uuid};HttpOnly;Secure;SameSite=Strict"))
        .or_internal_server_error(s_t!(i18n, "sid-gen-err"))
}

/// log out of the competition,
/// `wipe_progress` if requested,
/// otherwise preserve team progress for future relogins
///
/// returns empty, expired `sid` `SetCookie` header => browser deletes the valid one => user's now deauthed
///
/// WARN: **always** returns `Some(Ok(SetHeader { data: None }))`, see <https://github.com/DioxusLabs/dioxus/issues/5089>
#[post("/api/logout", headers: HeaderMap, cookies: CookieMap)]
pub async fn logout(wipe_progress: Option<bool>) -> Result<SetHeader<SetCookie>, HttpError> {
    let i18n = Localizer::from_headers(&headers);
    check_admin_pwd(&i18n)?;
    let uuid = extract_sid_cookie_localized(cookies, &i18n).await?;

    if wipe_progress.is_some_and(|sure| sure) {
        let username = USER_IDS
            .read()
            .await
            .get(&uuid)
            .or_not_found(s_t!(i18n, "no-progress-no-wipe"))?
            .clone();
        info!(
            "{}",
            s_tid!(i18n, "wiping-progress-of", name: username.clone())
        );
        _ = TEAMS.write().await.remove(&username);
    }

    _ = USER_IDS
        .write()
        .await
        .remove(&uuid)
        .or_not_found(s_t!(i18n, "invalid-sid-no-logout"))?;

    #[cfg(feature = "server_state_save")]
    tokio::spawn(state_save::save_state());

    // this makes the client invalidate the actual sid cookie
    SetHeader::new("sid=;Expires=Thu, 01 Jan 1970 00:00:00 GMT")
        .or_internal_server_error(s_t!(i18n, "sid-gen-err"))
}

/// before this, no solution can be set, no state will be loaded
/// NOTE: might take a while, as it hashes the `password` and loads the state
/// NOTE: use https
#[post("/api/set_admin_password", headers: HeaderMap)]
pub async fn set_admin_password(mut password: String) -> Result<String, HttpError> {
    let i18n = Localizer::from_headers(&headers);
    check_admin_pwd(&i18n)
        .is_err()
        .or_forbidden(s_t!(i18n, "admin-password-already-set"))?;

    let hashed_key = match argon2::hash_raw(password.as_bytes(), &*SALT, &ARGON2CONF) {
        Ok(hk) => hk,
        Err(e) => HttpError::internal_server_error(s_tid!(
            i18n,
            "hash-err",
            error: e.to_string()
        ))?,
    };

    #[cfg(feature = "server_state_save")]
    if let Err(err) = state_save::load_state(password.as_bytes(), &i18n).await {
        return HttpError::internal_server_error(s_tid!(
            i18n,
            "state-load-err",
            error: err.to_string()
        ));
    }
    password.zeroize();

    _ = HASHED_PWD.set(hashed_key); // NOTE: safe to ignore, as `is_none`, see above

    Ok(s_t!(i18n, "admin-password-set-success"))
}

/// set `puzzle_solutions` with `ADMIN_PASSWORD`
///
/// NOTE: if any of the solutions is incorrect, none will be saved
#[post("/api/set_solution", headers: HeaderMap)]
pub async fn set_solution(
    puzzle_solutions: PuzzleSolutions,
    mut password: String,
) -> Result<String, HttpError> {
    let i18n = Localizer::from_headers(&headers);
    // submitting as admin
    let hashed_key = check_admin_pwd(&i18n)?;
    let pwd_matches = argon2::verify_raw(password.as_bytes(), &*SALT, hashed_key, &ARGON2CONF)
        .inspect_err(|e| {
            error!(
                "{}",
                s_tid!(i18n, "password-verify-err", error: e.to_string())
            )
        })
        .or_internal_server_error(s_tid!(
            i18n,
            "password-verify-err",
            error: "unknown"
        ))?;
    password.zeroize();
    pwd_matches.or_unauthorized(s_t!(i18n, "incorrect-password"))?;

    let puzzles_lock = PUZZLES.read().await;
    puzzle_solutions
        .keys()
        .any(|new_k| !puzzles_lock.contains_key(new_k))
        .or_forbidden(s_t!(i18n, "puzzles-already-set"))?;
    drop(puzzles_lock);

    PUZZLES.write().await.extend(puzzle_solutions);

    #[cfg(feature = "server_state_save")]
    tokio::spawn(state_save::save_state());

    Ok(s_t!(i18n, "set-solution-success"))
}

/// submit a solution as a team
///
/// We'll use the `TypedHeader` extractor on the server to get the cookie from the request.
#[post("/api/submit", headers: HeaderMap, cookies: CookieMap)]
pub async fn submit_solution(
    puzzle_id: PuzzleId,
    solution: PuzzleSolution,
) -> Result<String, HttpError> {
    let i18n = Localizer::from_headers(&headers);
    check_admin_pwd(&i18n)?;
    let uuid = extract_sid_cookie_localized(cookies, &i18n).await?;
    let username = USER_IDS
        .read()
        .await
        .get(&uuid)
        .or_not_found(s_t!(i18n, "uuid-no-team"))?
        .clone(); // PERF: rather clone than lock

    PUZZLES
        .read()
        .await
        .get(&puzzle_id)
        .or_not_found(s_t!(i18n, "no-puzzle"))?
        .solution
        .eq(&solution)
        .or_forbidden(s_t!(i18n, "incorrect-puzzle-solution"))?;

    let mut teams_lock = TEAMS.write().await;

    let team_solved = teams_lock
        .get_mut(&username)
        .or_internal_server_error(s_t!(i18n, "team-progress-missing"))?;

    team_solved
        .insert(puzzle_id)
        .or_forbidden(s_t!(i18n, "puzzle-already-solved"))?;
    drop(teams_lock);

    #[cfg(feature = "server_state_save")]
    tokio::spawn(state_save::save_state());

    Ok(s_t!(i18n, "puzzle-submit-success"))
}
