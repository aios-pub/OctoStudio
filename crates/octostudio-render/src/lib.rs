//! OctoStudio v0.6 — render crate now only owns the [`AppState`] struct and
//! helper functions; the 6 screen widget trees were folded into
//! `octostudio_app::app` in C3 because `script_mod!` macro widgets are
//! lexically scoped and cannot be cross-referenced across modules.
//!
//! Modules retained for forward compatibility (will be re-introduced as
//! pure-Rust helpers / builders once the makepad widget API stabilises
//! for cross-module use):
//! - [`home`], [`compose`], [`studio`], [`plan`], [`export`], [`settings`]
//!   — currently each declares one widget via `script_mod!` for standalone
//!   testing / live-reload previews; the main app inlines the bodies.
//! - [`state`] — [`AppState`] (the 35+ field mirror of the splash `let`s).

pub mod compose;
pub mod export;
pub mod home;
pub mod plan;
pub mod settings;
pub mod state;
pub mod studio;

pub use state::AppState;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
