use super::{CookieMap, models::*};
use crate::backend::i18n::Localizer;
use crate::s_t;
use dioxus::prelude::*;
use rand_core::{OsRng, RngCore};
use std::sync::{LazyLock, OnceLock};
use std::{collections::HashMap, env};
use tokio::sync::RwLock;
use uuid::Uuid;

/// who's joined -> their name
type Teams = HashMap<Uuid, String>;
pub(super) static USER_IDS: LazyLock<RwLock<Teams>> = LazyLock::new(|| RwLock::new(Teams::new()));

pub(super) static PUZZLES: LazyLock<RwLock<PuzzleSolutions>> =
    LazyLock::new(|| RwLock::new(PuzzleSolutions::new()));

pub(super) static TEAMS: LazyLock<RwLock<TeamsState>> =
    LazyLock::new(|| RwLock::new(TeamsState::new()));

// SECURITY: it's fine like this, right?
pub(super) static SALT: LazyLock<[u8; 32]> = LazyLock::new(gen_salt);

pub(super) static ARGON2CONF: LazyLock<argon2::Config> = LazyLock::new(argon2::Config::default);
pub(super) static HASHED_PWD: OnceLock<Vec<u8>> = OnceLock::new();
/// initial, generated password that's required to set actual admin-password [`HASHED_PWD`]
pub static INIT_PWD: LazyLock<String> = LazyLock::new(|| Uuid::new_v4().to_string());
pub(super) static EVENT_TITLE: LazyLock<Result<String, env::VarError>> =
    LazyLock::new(|| env::var("APOLLO_EVENT_TITLE"));

/// check whether admin password was set
pub(super) fn check_admin_pwd(i18n: &Localizer) -> Result<&'static Vec<u8>, HttpError> {
    HASHED_PWD
        .get()
        .or_forbidden(s_t!(i18n, "admin-password-not-set"))
}

fn gen_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub(super) fn hash_puzzle_solution(raw_solution: &str) -> Result<PuzzleSolutionHash, HttpError> {
    argon2::hash_encoded(raw_solution.as_bytes(), &gen_salt(), &ARGON2CONF)
        .inspect_err(|e| error!("nem sikerült hasítani egy feladatmegoldást: {e}"))
        .or_internal_server_error("nem sikerült hasítani egy feladatmegoldást")
}

/// get a clone of state: `TEAMS` and `PUZZLES`
pub(super) async fn get_game_state() -> (TeamsState, PuzzlesExisting) {
    let existing_puzzles = PUZZLES
        .read()
        .await
        .clone()
        .into_iter()
        .map(|(id, pzl)| (id, pzl.value))
        .collect();
    (TEAMS.read().await.clone(), existing_puzzles)
}

pub(super) async fn extract_sid_cookie_localized(
    cookies: CookieMap,
    i18n: &Localizer,
) -> Result<Uuid, HttpError> {
    let uuid = cookies
        .get("sid")
        .or_unauthorized(s_t!(i18n, "sid-missing"))?;
    Uuid::try_from(uuid).or_bad_request(s_t!(i18n, "sid-invalid"))
}

#[cfg(feature = "server_state_save")]
pub(super) mod state_save {
    use super::{PUZZLES, SALT, TEAMS, Teams, USER_IDS, check_admin_pwd};
    use crate::backend::i18n::Localizer;
    use crate::backend::models::*;
    use crate::{s_t, s_tid};
    use chacha20poly1305::aead::{Aead, Nonce, OsRng};
    use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305};
    use dioxus::prelude::*;
    use std::{env, path::Path, sync::LazyLock};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use zeroize::Zeroize;

    type Res<T> = Result<T, Box<dyn std::error::Error>>;
    /// state that's stored on disk
    type StateOnDisk = (TeamsState, PuzzleSolutions, Teams);

    static STATE_PATH: LazyLock<String> = LazyLock::new(|| {
        let i18n = Localizer::fallback_hu();
        let def = String::from("apollo-state.cbor.encrypted"); // WARN: might not exist...
        let path = env::var("APOLLO_STATE_PATH")
            .inspect_err(|e| {
                warn!(
                    "{}",
                    s_tid!(
                        i18n,
                        "state-path-env-missing",
                        error: e.to_string(),
                        default: format!("{def:?}")
                    )
                )
            })
            .unwrap_or_else(|_| def.clone());
        if path.is_empty() { def } else { path }
    });

    async fn encrypt(raw_content: &[u8]) -> Res<Vec<u8>> {
        let hashed_key = check_admin_pwd(&Localizer::fallback_hu())?;
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let cipher = XChaCha20Poly1305::new(hashed_key.as_slice().into());
        let encrypted_content = cipher.encrypt(&nonce, raw_content).map_err(|e| {
            s_tid!(
                &Localizer::fallback_hu(),
                "encryption-err",
                error: e.to_string()
            )
        })?;

        let mut buf = Vec::with_capacity(SALT.len() + nonce.len() + encrypted_content.len());

        buf.write_all(&*SALT).await?;
        buf.write_all(&nonce).await?;
        buf.write_all(&encrypted_content).await?;

        Ok(buf)
    }

    pub async fn decrypt_state(
        encrypted_path: impl AsRef<Path>,
        raw_pwd: &[u8],
        i18n: &Localizer,
    ) -> Res<Vec<u8>> {
        let mut salt = [0u8; 32];
        let mut nonce = Nonce::<XChaCha20Poly1305>::default();

        let encrypted_path = encrypted_path.as_ref();
        let mut encrypted_file = tokio::fs::File::open(encrypted_path).await?;

        let mut read_count = encrypted_file.read(&mut salt).await?;
        if read_count != salt.len() {
            return Err(s_t!(i18n, "salt-read-err").into());
        }

        read_count = encrypted_file.read(&mut nonce).await?;
        if read_count != nonce.len() {
            return Err(s_t!(i18n, "nonce-read-err").into());
        }

        let mut derived_key = argon2::hash_raw(raw_pwd, &salt, &super::ARGON2CONF)?;

        let cipher = XChaCha20Poly1305::new(derived_key.as_slice().into());
        let mut buf = vec![];
        let _n = encrypted_file.read_to_end(&mut buf).await?;

        let decrypted_content = cipher.decrypt(&nonce, buf.as_slice()).map_err(|e| {
            s_tid!(
                i18n,
                "decryption-err",
                path: format!("{encrypted_path:?}"),
                error: e.to_string()
            )
        })?;

        salt.zeroize();
        nonce.zeroize();
        derived_key.zeroize();

        Ok(decrypted_content)
    }

    /// save `PUZZLES` and `TEAMS` state to disk into an encrypted `cbor` file
    /// logs errors to server stderr
    pub async fn save_state() {
        if let Err(err) = _save_state().await {
            error!(
                "{}",
                s_tid!(
                    Localizer::fallback_hu(),
                    "state-save-failed",
                    error: err.to_string()
                )
            );
        }
    }

    async fn _save_state() -> Result<(), HttpError> {
        // internal server error
        let ise = |msg: String| HttpError::new(StatusCode::INTERNAL_SERVER_ERROR, msg);
        let teams_state = TEAMS.read().await.clone();
        let puzzles_state = PUZZLES.read().await.clone();
        let userid_state = USER_IDS.read().await.clone();
        let disk_state: StateOnDisk = (teams_state, puzzles_state, userid_state);

        let mut state_buf = vec![];
        ciborium::into_writer(&disk_state, &mut state_buf).map_err(|e| {
            ise(s_tid!(
                Localizer::fallback_hu(),
                "state-serialisation-err",
                error: e.to_string()
            ))
        })?;
        let encrypted_state = encrypt(&state_buf).await.map_err(|e| {
            ise(s_tid!(
                Localizer::fallback_hu(),
                "state-encryption-err",
                error: e.to_string()
            ))
        })?;
        tokio::fs::write(&*STATE_PATH, encrypted_state)
            .await
            .map_err(|e| {
                ise(s_tid!(
                    Localizer::fallback_hu(),
                    "state-write-err",
                    path: format!("{STATE_PATH:?}"),
                    error: e.to_string()
                ))
            })?;

        Ok(())
    }

    /// load state from `STATE_PATH` into memory if it exists
    pub async fn load_state(raw_pwd: &[u8], i18n: &Localizer) -> Res<()> {
        if !tokio::fs::try_exists(&*STATE_PATH).await? {
            warn!(
                "{}",
                s_tid!(i18n, "state-file-missing", path: format!("{STATE_PATH:?}"))
            );
            return Ok(()); // no need to load, it's fine
        }
        let (teams_state, puzzles_state, userid_state): StateOnDisk = {
            let encrypted_data = decrypt_state(&*STATE_PATH, raw_pwd, i18n).await?;
            ciborium::from_reader(encrypted_data.as_slice())?
        };
        PUZZLES.write().await.extend(puzzles_state);
        TEAMS.write().await.extend(teams_state);
        USER_IDS.write().await.extend(userid_state);
        info!(
            "{}",
            s_tid!(i18n, "state-load-success", path: format!("{STATE_PATH:?}"))
        );
        Ok(())
    }
}
