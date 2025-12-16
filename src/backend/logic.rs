use super::models::*;
use chacha20poly1305::aead::{OsRng, rand_core::RngCore};
use dioxus::fullstack::{Cookie, TypedHeader};
use dioxus::prelude::*;
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
pub(super) static SALT: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
});
pub(super) static ARGON2CONF: LazyLock<argon2::Config> = LazyLock::new(argon2::Config::default);
pub(super) static HASHED_PWD: OnceLock<Vec<u8>> = OnceLock::new();
pub(super) static EVENT_TITLE: LazyLock<Result<String, env::VarError>> =
    LazyLock::new(|| env::var("APOLLO_EVENT_TITLE"));

/// check whether admin password was set
pub(super) fn check_admin_pwd() -> Result<&'static Vec<u8>, HttpError> {
    HASHED_PWD
        .get()
        .or_forbidden("még nincs beállítva mesterjelszó")
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

/// extract session id cookie from cookie headers
pub(super) async fn extract_sid_cookie(cookies: TypedHeader<Cookie>) -> Result<Uuid, HttpError> {
    let uuid = cookies
        .get("sid")
        .or_unauthorized("hiányzik a munkamenet-azonosító süti")?;
    Uuid::try_from(uuid).or_bad_request("érvénytelen munkamenet-azonosító süti")
}

#[cfg(feature = "server_state_save")]
pub(super) mod state_save {
    use super::{HASHED_PWD, PUZZLES, SALT, TEAMS, Teams, USER_IDS};
    use crate::backend::models::*;
    use chacha20poly1305::aead::{Aead, Nonce, OsRng};
    use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305};
    use dioxus::prelude::*;
    use secrecy::zeroize::Zeroize; // TODO: either deal with secrecy, or use zeroize itself
    use std::{env, path::Path, sync::LazyLock};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    type Res<T> = Result<T, Box<dyn std::error::Error>>;
    /// state that's stored on disk
    type StateOnDisk = (TeamsState, PuzzleSolutions, Teams);

    static STATE_PATH: LazyLock<String> = LazyLock::new(|| {
        let def = String::from("apollo-state.cbor.encrypted"); // WARN: might not exist...
        let path = env::var("APOLLO_STATE_PATH")
            .inspect_err(|e| warn!("nincs beallitva az állapot mentési helye: {e}"))
            .unwrap_or_else(|_| def.clone());
        if path.is_empty() { def } else { path }
    });

    async fn encrypt(raw_content: &[u8]) -> Res<Vec<u8>> {
        let hashed_key = HASHED_PWD.get().ok_or("nincs még beállítva mesterjelszó")?;
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let cipher = XChaCha20Poly1305::new(hashed_key.as_slice().into());
        let encrypted_content = cipher
            .encrypt(&nonce, raw_content)
            .map_err(|e| format!("nem sikerült a titkosítás: {e}"))?;

        let mut buf = Vec::with_capacity(SALT.len() + nonce.len() + encrypted_content.len());

        buf.write_all(&*SALT).await?;
        buf.write_all(&nonce).await?;
        buf.write_all(&encrypted_content).await?;

        Ok(buf)
    }

    pub async fn decrypt_state(encrypted_path: impl AsRef<Path>, raw_pwd: &[u8]) -> Res<Vec<u8>> {
        let mut salt = [0u8; 32];
        let mut nonce = Nonce::<XChaCha20Poly1305>::default();

        let encrypted_path = encrypted_path.as_ref();
        let mut encrypted_file = tokio::fs::File::open(encrypted_path).await?;

        let mut read_count = encrypted_file.read(&mut salt).await?;
        if read_count != salt.len() {
            return Err("nem sikerült kiolvasni a sót".into());
        }

        read_count = encrypted_file.read(&mut nonce).await?;
        if read_count != nonce.len() {
            return Err("nem sikerült kiolvasni az alkalmi kifejezést".into());
        }

        let mut derived_key = argon2::hash_raw(raw_pwd, &salt, &super::ARGON2CONF)?;

        let cipher = XChaCha20Poly1305::new(derived_key.as_slice().into());
        let mut buf = vec![];
        let _n = encrypted_file.read_to_end(&mut buf).await?;

        let decrypted_content = cipher
            .decrypt(&nonce, buf.as_slice())
            .map_err(|e| format!("nem sikerült visszafejteni a fajlt({encrypted_path:?}), győződj meg róla, hogy ugyanazzal a jelszóval próbálkozol, amivel titkosítva lett: {e}"))?;

        salt.zeroize();
        nonce.zeroize();
        derived_key.zeroize();

        Ok(decrypted_content)
    }

    /// save `PUZZLES` and `TEAMS` state to disk into an encrypted `cbor` file
    /// logs errors to server stderr
    pub async fn save_state() {
        if let Err(err) = _save_state().await {
            error!("nem sikerült elmenteni az állapotot: {err}");
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
        ciborium::into_writer(&disk_state, &mut state_buf)
            .map_err(|e| ise(format!("nem sikerült cbor-rá alakítani az állapotot: {e}")))?;
        let encrypted_state = encrypt(&state_buf)
            .await
            .map_err(|e| ise(format!("nem sikerült titkosítani az állapot: {e}")))?;
        tokio::fs::write(&*STATE_PATH, encrypted_state)
            .await
            .map_err(|e| {
                ise(format!(
                    "nem sikerült az állapotot fájlba({STATE_PATH:?}) írni: {e}"
                ))
            })?;

        Ok(())
    }

    /// load state from `STATE_PATH` into memory if it exists
    pub async fn load_state(raw_pwd: &[u8]) -> Res<()> {
        if !tokio::fs::try_exists(&*STATE_PATH).await? {
            warn!("nem létezik a megadott állapot-fájl({STATE_PATH:?})");
            return Ok(()); // no need to load, it's fine
        }
        let (teams_state, puzzles_state, userid_state): StateOnDisk = {
            let encrypted_data = decrypt_state(&*STATE_PATH, raw_pwd).await?;
            ciborium::from_reader(encrypted_data.as_slice())?
        };
        PUZZLES.write().await.extend(puzzles_state);
        TEAMS.write().await.extend(teams_state);
        USER_IDS.write().await.extend(userid_state);
        info!("sikeresen betöltöttük az elmentett állapotot fájlból({STATE_PATH:?}) a memóriába");
        Ok(())
    }
}
