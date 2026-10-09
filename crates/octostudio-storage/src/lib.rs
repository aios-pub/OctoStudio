//! OctoStudio v0.6 — persistent storage layer.
//!
//! C4 fills this with:
//! - [`paths`] — resolve `works.json` / `config.json` / `usage.json`
//!   via the `dirs` crate (macOS: `~/Library/Application Support/octostudio/`,
//!   Linux: `~/.local/share/octostudio/`, Windows: `%APPDATA%\octostudio\`).
//! - [`works`] — load + save the `Vec<Work>` (works.json).
//! - [`config`] — load + save the user `Config` (API key, theme, etc).
//! - [`usage`] — daily token aggregation (usage.json).
//! - [`migration`] — read legacy splash `<bundle>/.local-state/works.json`
//!   and merge `plan_sections` / `plan_scenes` into the new `plan_items`.

pub mod config;
pub mod migration;
pub mod paths;
pub mod usage;
pub mod works;

pub use config::Config;
pub use octostudio_core::{Composition, HistoryEntry, PlanItem, Work};
pub use paths::{config_path, ensure_data_dir, usage_path, works_path};
pub use usage::{UsageDay, UsageRecord};
pub use works::{load_works, save_works};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
