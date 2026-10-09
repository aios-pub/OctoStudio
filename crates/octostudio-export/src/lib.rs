//! Export format renderers for OctoStudio v0.6.
//!
//! C8 will fill this with the 8 format renderers ported 1:1 from the splash
//! `rt_*` functions:
//! - `markdown::rt_image / rt_video / rt_ppt / rt_rows / rt_titles / rt_xhs`
//! - `wechat::*`  (plain-text variants of the markdown renderers)
//! - `notion::*`  (markdown with inline code blocks for image prompts)
//! - `srt::srt_text()`  (HH:MM:SS,000 time-coded subtitles)
//! - `pack::pack_text()`  (video production pack with ffmpeg concat cmd)
//! - `marp::marp_text()`  (Marp frontmatter + per-slide bullets + notes)
//! - `markmap::markmap_text()`  (root + branch list)
//! - `dispatch::render_plan_as_text(plan_kind, plan_format, …)`  (central router)

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
