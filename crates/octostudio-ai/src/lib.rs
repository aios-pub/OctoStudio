//! OctoStudio v0.6 — real AI clients (OpenAI-compatible).
//!
//! C5 fills [`text`] with the Agnes 3.0 Flash client + 7 panel helpers +
//! 10 scenario schemas. C6 adds [`image`] (Agnes 2.5 Flash); C7 adds
//! [`video`] (Agnes 2.5 + async polling).
//!
//! All three clients reuse [`client::HttpClient`] — a thin `ureq`
//! wrapper that:
//! - injects the `Authorization: Bearer <api_key>` header,
//! - serialises/deserialises JSON via `serde_json`,
//! - maps errors to [`octostudio_core::OctostudioError::Ai`] /
//!   [`octostudio_core::OctostudioError::Http`].

pub mod client;
pub mod prompts;
pub mod text;

pub use client::{HttpClient, HttpResponse};
pub use text::{ChatMessage, ChatRequest, ChatResponse, TextClient};

/// Agnes API hub. Override with `OCTOSTUDIO_AGNES_BASE_URL` for testing.
pub const DEFAULT_BASE_URL: &str = "https://apihub.agnes-ai.com/v1";

/// Default model for chat completions.
pub const TEXT_MODEL: &str = "agnes-3.0-flash";

/// Default model for image generation.
pub const IMAGE_MODEL: &str = "agnes-image-2.5-flash";

/// Default model for video generation.
pub const VIDEO_MODEL: &str = "agnes-video-2.5";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
