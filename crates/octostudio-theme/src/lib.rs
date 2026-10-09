//! OctoStudio v0.6 design tokens — color and typography, ported 1:1 from
//! the splash bundle (v0.5-alpha-ui-polish).
//!
//! - [`color`] — 15 light-mode color tokens (ink / secondary / accent /
//!   accent_hover / accent_soft / warm_hover / card_bg / chip_bg / canvas /
//!   surface / hairline / danger / success / warning / info) + 4 dark-mode
//!   reservations. Hex string format is the same as the splash `#xRRGGBB`,
//!   so a single string can be re-used in `script_mod!` blocks.
//! - [`font`]  — 7 type-scale tokens, 11→12→14→16→18→22→26 pt (caption /
//!   meta / body / body_lg / section / title / hero).

pub mod color;
pub mod font;

pub use color::*;
pub use font::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
