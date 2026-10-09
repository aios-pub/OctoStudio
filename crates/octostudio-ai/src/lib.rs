//! Real AI clients for OctoStudio v0.6. All OpenAI-compatible.
//!
//! C5/C6/C7 will populate this with:
//! - `text::TextClient` — POST /v1/chat/completions (model: agnes-3.0-flash)
//!   7 panel helpers: ai_gen_title, ai_extract_keywords, ai_summarize,
//!   ai_style_variants, ai_translate_zh_en, ai_score_title, ai_budget (local)
//!   10 scenario schemas: ask_scenario(PlanKind, task, input)
//! - `image::ImageClient` — POST /v1/images/generations (model: agnes-image-2.5-flash)
//! - `video::VideoClient` — POST /v1/videos (model: agnes-video-2.5) + async poll
//!   GET /agnesapi?video_id=...&model_name=...
//! - `prompts::SCENARIO_TASKS` (10), `prompts::SCENARIO_SCHEMAS` (10)

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
