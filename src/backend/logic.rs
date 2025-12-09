use super::models::*;
use dioxus::fullstack::{Cookie, TypedHeader};
use dioxus::prelude::*;
use secrecy::SecretString;
use std::sync::LazyLock;
use std::{collections::HashMap, env, process};
use tokio::sync::RwLock;
use uuid::Uuid;

type Teams = HashMap<uuid::Uuid, String>;
pub(super) static USER_IDS: LazyLock<RwLock<Teams>> = LazyLock::new(|| RwLock::new(Teams::new()));

pub(super) static PUZZLES: LazyLock<RwLock<PuzzleSolutions>> =
    LazyLock::new(|| RwLock::new(PuzzleSolutions::new()));

pub(super) static TEAMS: LazyLock<RwLock<TeamsState>> =
    LazyLock::new(|| RwLock::new(TeamsState::new()));

/// without `key`, the app won't run
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
/// - if necessary admin env vars aren't set
/// - if should load state but can't
pub async fn prepare_startup() {
    _ = LazyLock::force(&ADMIN_PASSWORD);
    #[cfg(feature = "server_state_save")]
    if let Err(e) = state_save::load_state().await {
        error!("couldn't load state: {e}, exiting...");
        process::exit(1);
    }
}

pub(super) static ADMIN_PASSWORD: LazyLock<SecretString> =
    LazyLock::new(|| ensure_env_var("APOLLO_MESTER_JELSZO").into());
pub(super) static EVENT_TITLE: LazyLock<Result<String, env::VarError>> =
    LazyLock::new(|| env::var("APOLLO_EVENT_TITLE"));

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

pub(super) async fn extract_session_id_cookie(
    cookies: TypedHeader<Cookie>,
) -> Result<Uuid, HttpError> {
    let uuid = cookies
        .get("session_id")
        .or_unauthorized("missing session_id cookie")?;
    Uuid::try_from(uuid).or_bad_request("invalid session_id cookie")
}

#[cfg(feature = "server_state_save")]
pub(super) mod state_save {
    use super::{ADMIN_PASSWORD, PUZZLES, TEAMS, Teams, USER_IDS};
    use crate::backend::models::*;
    use chacha20poly1305::aead::{Aead, Nonce, OsRng, rand_core::RngCore};
    use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305};
    use dioxus::{fullstack::serde, prelude::*};
    use secrecy::{ExposeSecret, zeroize::Zeroize};
    use std::{path::Path, process, sync::LazyLock};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    type Res<T> = Result<T, Box<dyn std::error::Error>>;
    type StateOnDisk = (TeamsState, PuzzleSolutions, Teams);

    static STATE_PATH: LazyLock<String> =
        LazyLock::new(|| super::ensure_env_var("APOLLO_STATE_PATH"));

    static ARGON2CONF: LazyLock<argon2::Config> = LazyLock::new(argon2::Config::default);
    static SALT: LazyLock<[u8; 32]> = LazyLock::new(|| {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        salt
    });
    // SECURITY: should it be generated on each save?
    static NONCE: LazyLock<Nonce<XChaCha20Poly1305>> =
        LazyLock::new(|| XChaCha20Poly1305::generate_nonce(&mut OsRng));
    static DERIVED_KEY: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let Ok(derived_key) = argon2::hash_raw(
            ADMIN_PASSWORD.expose_secret().as_bytes(),
            &*SALT,
            &ARGON2CONF,
        )
        .inspect_err(|e| error!("couldn't hash password: {e}")) else {
            process::exit(1);
        };
        derived_key
    });

    async fn encrypt(raw_content: &[u8]) -> Res<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new(DERIVED_KEY.as_slice().into());
        let encrypted_content = cipher
            .encrypt(&NONCE, raw_content)
            .map_err(|e| format!("encryption error: {e}"))?;

        let mut buf = Vec::with_capacity(SALT.len() + NONCE.len() + encrypted_content.len());

        buf.write_all(&*SALT).await?;
        buf.write_all(&NONCE).await?;
        buf.write_all(&encrypted_content).await?;

        Ok(buf)
    }

    async fn decrypt_state(encrypted_path: impl AsRef<Path>) -> Res<Vec<u8>> {
        let mut salt = [0u8; 32];
        let mut nonce = Nonce::<XChaCha20Poly1305>::default();

        let mut encrypted_file = tokio::fs::File::open(encrypted_path).await?;

        let mut read_count = encrypted_file.read(&mut salt).await?;
        if read_count != salt.len() {
            return Err("couldn't read salt".into());
        }

        read_count = encrypted_file.read(&mut nonce).await?;
        if read_count != nonce.len() {
            return Err("couldn't read nonce".into());
        }

        let mut derived_key = argon2::hash_raw(
            ADMIN_PASSWORD.expose_secret().as_bytes(),
            &salt,
            &ARGON2CONF,
        )?;

        let cipher = XChaCha20Poly1305::new(derived_key.as_slice().into());
        let mut buf = vec![];
        let _n = encrypted_file.read_to_end(&mut buf).await?;

        let decrypted_content = cipher
            .decrypt(&nonce, buf.as_slice())
            .map_err(|e| format!("error decrypting file, make sure you're trying to decrypt it with the same password that was used for it's encryption: {e}"))?;

        salt.zeroize();
        nonce.zeroize();
        derived_key.zeroize();

        Ok(decrypted_content)
    }

    /// save `PUZZLES` and `TEAMS` state to disk into an encrypted `cbor` file
    pub async fn save_state() -> Result<(), HttpError> {
        // internal server error
        let ise = |msg: String| HttpError::new(StatusCode::INTERNAL_SERVER_ERROR, msg);
        let teams_state = TEAMS.read().await.clone();
        let puzzles_state = PUZZLES.read().await.clone();
        let userid_state = USER_IDS.read().await.clone();
        let disk_state: StateOnDisk = (teams_state, puzzles_state, userid_state);

        let mut state_buf = vec![];
        ciborium::into_writer(&disk_state, &mut state_buf)
            .map_err(|e| ise(format!("couldn't serialize state to cbor: {e}")))?;
        let encrypted_state = encrypt(&state_buf)
            .await
            .map_err(|e| ise(format!("couldn't encrypt state: {e}")))?;
        tokio::fs::write(&*STATE_PATH, encrypted_state)
            .await
            .map_err(|e| ise(format!("couldn't write state to file: {e}")))?;

        Ok(())
    }

    async fn load_state_of<D: for<'de> serde::Deserialize<'de>, P: AsRef<Path>>(path: P) -> Res<D> {
        let encypted_data = decrypt_state(path).await?;
        let state = ciborium::from_reader(encypted_data.as_slice())?;
        Ok(state)
    }

    /// load state from `STATE_PATH` into memory if it exists
    pub(super) async fn load_state() -> Res<()> {
        if !tokio::fs::try_exists(&*STATE_PATH).await? {
            return Ok(()); // no need to load, it's fine
        }
        let (teams_state, puzzles_state, userid_state): StateOnDisk =
            load_state_of(&*STATE_PATH).await?;
        PUZZLES.write().await.extend(puzzles_state);
        TEAMS.write().await.extend(teams_state);
        USER_IDS.write().await.extend(userid_state);
        info!("successfully loaded saved state from {STATE_PATH:?} to memory");
        Ok(())
    }
}
