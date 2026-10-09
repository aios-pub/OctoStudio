//! Boot — load `works.json` + `config.json`, run splash migration,
//! fall back to demo data. Called once from [`crate::app::App::handle_startup`].
//!
//! Returns `(works, config)` so the caller can hydrate [`AppState`].
//! The function never panics: any IO or parse error degrades to
//! demo data + default config, so a corrupt `works.json` can't brick
//! the app.

use octostudio_core::OctostudioResult;
use octostudio_core::Work;
use octostudio_demo::demo_works;
use octostudio_storage::config::{load_config, Config};
use octostudio_storage::migration;
use octostudio_storage::works::{load_works, save_works};

/// Load works + config. Always succeeds (errors are swallowed with
/// `eprintln!` so the user sees them in stderr but the app boots).
pub fn boot() -> (Vec<Work>, Config) {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[boot] config load failed: {e} — using defaults");
            Config::default()
        }
    };

    let works = match load_works() {
        Ok(ws) if !ws.is_empty() => ws,
        Ok(_) => {
            // File exists but empty — first launch. Try migration first.
            let migrated = migration::migrate_from_splash();
            if !migrated.is_empty() {
                if let Err(e) = save_works(&migrated) {
                    eprintln!("[boot] saving migrated works failed: {e}");
                }
                migrated
            } else {
                let demo = demo_works();
                if let Err(e) = save_works(&demo) {
                    eprintln!("[boot] saving demo works failed: {e}");
                }
                demo
            }
        }
        Err(e) => {
            eprintln!("[boot] works load failed: {e} — using demo");
            demo_works()
        }
    };

    (works, config)
}

/// One-shot smoke test helper: round-trip works through load/save.
pub fn roundtrip_works_smoke() -> OctostudioResult<()> {
    let works = load_works()?;
    save_works(&works)?;
    Ok(())
}
