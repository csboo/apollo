//! based on
//! - <https://stackoverflow.com/questions/43753491/include-git-commit-hash-as-string-into-rust-program>
//! - <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
//! - <https://doc.rust-lang.org/cargo/reference/build-scripts.html>

use std::process::Command;
use std::time::SystemTime;
use std::{env, io};

fn main() {
    let git_commit_hash = match git_commit_hash() {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!(
                "WARN: you probably have no `git` installed, couldn't retrieve commit hash: {e}"
            );
            String::from("<unknown>")
        }
    };

    let banner_msg = format!(
        "
---------------- {} ----------------
{}
version: {} ({} build for {})
built on {} from commit {}
enabled features: [{}]
repo: {}
----------------------------------------
",
        env!("CARGO_PKG_NAME").to_uppercase(),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
        env::var("PROFILE").unwrap(),
        env::var("TARGET").unwrap(),
        humantime::format_rfc3339_seconds(SystemTime::now()),
        git_commit_hash.trim(),
        env::var("CARGO_CFG_FEATURE").unwrap(),
        env!("CARGO_PKG_REPOSITORY"),
    );

    println!("cargo:rustc-env=BANNER={banner_msg:?}"); // escape newlines and quote it
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");
}

/// get current git commit hash
/// # Errors
/// if can't, i.d. no `git` installed
fn git_commit_hash() -> io::Result<String> {
    let git_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()?;
    let git_commit_hash = String::from_utf8_lossy(&git_output.stdout);
    Ok(git_commit_hash.to_string())
}
