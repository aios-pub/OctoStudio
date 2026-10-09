//! Offline fallback demo data, ported 1:1 from the splash `demo_*` functions.
//!
//! C4 will populate this with:
//! - `plans::demo_article_sections()`        (image)
//! - `plans::demo_video_scenes()`            (video)
//! - `plans::demo_ppt_slides()`              (ppt)
//! - `plans::demo_teardown_items()`          (teardown)
//! - `plans::demo_titles_items()`            (titles)
//! - `plans::demo_xhs_items()`               (xhs)
//! - `plans::demo_script_items()`            (script)
//! - `plans::demo_mindmap_items()`           (mindmap)
//! - `plans::demo_quotes_items()`            (quotes)
//! - `plans::demo_composition()`             (video composition suggestion)
//! - `works::demo_works()`                   (2 works for first-launch demo)

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
