//! OctoStudio v0.6 — offline fallback demo data.
//!
//! C4 fills [`works`] with two demo works for first-launch users
//! (no API key, no legacy splash data). The 9 plan_kind datasets
//! land in C5 alongside the real text client.

pub mod works;

pub use works::demo_works;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
