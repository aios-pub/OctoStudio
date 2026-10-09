//! OctoStudio v0.6 domain types.
//!
//! Ported 1:1 from the splash bundle (v0.5-alpha-ui-polish) — every constant,
//! enum and data structure here corresponds to a splash `let` or array.
//!
//! Modules:
//! - [`enums`]   — [`Screen`], [`PlanKind`], [`ExportFormat`], [`SortMode`]
//! - [`work`]    — [`Work`], [`PlanItem`], [`Composition`], [`HistoryEntry`]
//! - [`constants`] — `SCENARIOS` (10), `THEMES` (11), `PRESETS` (6), `STYLES` (8)
//! - [`error`]  — [`OctostudioError`]

pub mod constants;
pub mod enums;
pub mod error;
pub mod fields;
pub mod work;

pub use constants::{PRESETS, SCENARIOS, STYLES, THEMES, Preset, Scenario, Style, Theme};
pub use enums::{ExportFormat, PlanKind, Screen, SortMode};
pub use error::{OctostudioError, OctostudioResult};
pub use fields::{blank_item, item_fields, ItemField};
pub use work::{Composition, HistoryEntry, PlanItem, Work};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
