//! Persistent storage for OctoStudio v0.6.
//!
//! C4 will fill this in with:
//! - `paths::works_path() / config_path() / usage_path()` resolved via `dirs`
//! - `works::load() / save()` for the works array
//! - `config::load() / save()` for API key + preferences
//! - `usage::today() / record()` for daily token aggregation
//! - `migration::from_splash()` to read legacy `<bundle>/.local-state/works.json`

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
