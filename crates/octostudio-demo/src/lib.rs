//! OctoStudio v0.6 — offline fallback demo data.
//!
//! - [`works`] — two first-launch demo works (测试作品一/二)
//! - [`plans`] — 9 plan_kind demo datasets (image/video/ppt/teardown/
//!   titles/xhs/script/mindmap/quotes + composition), ported 1:1
//!   from the splash `demo_*` functions. Used as the no-API-key
//!   fallback for `ask_scenario` / `ask_composition`.

pub mod plans;
pub mod works;

pub use plans::{demo_composition, load_demo_plan};
pub use works::demo_works;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
