#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

pub mod models;

#[cfg(feature = "server")]
mod i18n;
#[cfg(feature = "server")]
mod logic;
#[cfg(feature = "server")]
pub use logic::INIT_PWD;
#[cfg(feature = "server")]
pub type CookieMap = dioxus::fullstack::TypedHeader<dioxus::fullstack::Cookie>;

pub mod endpoints;
