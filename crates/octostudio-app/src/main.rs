//! OctoStudio native binary entry.
//!
//! v0.6 is a full rewrite from Splash to native Makepad (Rust) widgets.
//! - Splash VM, card-host, octosense, rinx are no longer supported.
//! - Real AI: Agnes 3.0 Flash (text), Agnes 2.5 Flash (image), Agnes 2.5 (video) —
//!   all OpenAI-compatible. See `octostudio-ai` crate.
//! - 5 screens (Home / Compose / Studio / Plan / Export) + Settings, 10 scenarios,
//!   8 export formats, 2 demo works, 9 plan-kind demo datasets.

pub use makepad_widgets;

use makepad_widgets::*;

mod app;
mod boot;
mod theme;

use crate::app::App;

app_main!(App);
