#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

pub mod models;

#[cfg(feature = "server")]
mod logic;

pub mod endpoints;
