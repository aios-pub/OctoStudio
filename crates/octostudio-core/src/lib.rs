//! Domain types and enums for OctoStudio v0.6.
//!
//! C2 will populate this crate with:
//! - `Work` (works.json entry)
//! - `PlanItem` (generic plan row with `_k` stable id)
//! - `Composition` (6-field video composition)
//! - `HistoryEntry` (AI history per work, max 3)
//! - `Screen` enum (Home | Compose | Studio | Plan | Export | Settings)
//! - `PlanKind` enum (None | Image | Video | Ppt | Teardown | Titles | Xhs | Script | Mindmap | Quotes)
//! - `ExportFormat` enum (Markdown | Wechat | Notion | Srt | Pack | Marp | Markmap)
//! - `SortMode` enum
//! - `UndoneEntry` enum
//! - `OctostudioError` (thiserror)
//! - Constants: SCENARIOS (10), THEMES (11), PRESETS (6), STYLES (8)

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
