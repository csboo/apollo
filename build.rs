//! based on
//! - <https://stackoverflow.com/questions/43753491/include-git-commit-hash-as-string-into-rust-program>
//! - <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
//! - <https://doc.rust-lang.org/cargo/reference/build-scripts.html>

use std::env;
use std::process::Command;
use std::time::SystemTime;

fn main() {
    let git_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap(); // NOTE: probably good enough for now
    let git_commit_hash = String::from_utf8_lossy(&git_output.stdout);

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

    println!("cargo:rustc-env=BANNER={banner_msg:?}");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");
}
