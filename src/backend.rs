#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

pub mod models;

#[cfg(feature = "server")]
mod logic;
#[cfg(feature = "server")]
pub use logic::INIT_PWD;

pub mod endpoints;
