//! OctoStudio v0.6 — 8 export format renderers.
//!
//! C8 fills this with the 8 format renderers ported 1:1 from the splash
//! `rt_*` / `srt_text` / `pack_text` / `marp_text` / `markmap_text`
//! functions. All renderers are pure string templates — no AI calls.
//!
//! - [`markdown`] — markdown variants of all 6 plan kinds
//! - [`wechat`]   — plain-text variants (公众号 / 小红书 plain)
//! - [`notion`]   — markdown with inline code blocks (for image_prompts)
//! - [`srt`]      — HH:MM:SS,000 time-coded subtitles
//! - [`pack`]     — video production pack (scene prompts + SRT + ffmpeg)
//! - [`marp`]     — Marp frontmatter + per-slide bullets + HTML-comment notes
//! - [`markmap`]  — root + branch list (compatible with markmap.js.org)
//! - [`dispatch`] — central router [`dispatch::render_plan_as_text`]

pub mod dispatch;
pub mod markdown;
pub mod markmap;
pub mod marp;
pub mod notion;
pub mod pack;
pub mod srt;
pub mod wechat;

pub use dispatch::render_plan_as_text;
pub use markmap::markmap_text;
pub use marp::marp_text;
pub use pack::pack_text;
pub use srt::srt_text;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
