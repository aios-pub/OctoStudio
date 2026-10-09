//! Native Makepad widgets for the 5 screens + Settings.
//!
//! C3 will fill this with the port of every Splash widget tree:
//! - `home::Home`     — works list, search, sort, multi-select, tag chips,
//!                      2x5 scenario grid, glance publish row
//! - `compose::Compose` — 10 scenario chips, theme/preset chips (per-kind), three
//!                         Card inputs (reference / intent / source), CTA
//! - `studio::Studio`   — 8 style chips, undo/snapshot, title/reference/source/
//!                         content inputs, AI assistant Card (7 chips + result)
//! - `plan::Plan`       — image toggle, plan title, plan list (≤16 items per
//!                         RoundedView with 4 chip actions), composition card
//! - `export::Export`   — 2-column format cards, big preview Card
//! - `settings::Settings` — API key input + "test connection" + theme picker +
//!                          usage stats

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
