#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

pub mod models;

#[cfg(feature = "server")]
pub use logic::prepare_startup;
#[cfg(feature = "server")]
mod logic;

pub mod endpoints;
