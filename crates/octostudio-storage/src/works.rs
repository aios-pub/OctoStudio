//! `works.json` IO.
//!
//! Splash used `fs.read("works.json")` against the bundle's `.local-state/`
//! directory. In v0.6 we resolve against the per-user `data_dir()` (see
//! [`crate::paths`]). The schema is byte-compatible with the splash
//! format — see [`crate::migration`] for one-time import.

use std::path::Path;

use octostudio_core::{OctostudioError, OctostudioResult, Work};

use crate::paths::works_path;

/// Load the works array from disk. Returns an empty `Vec` if the file
/// doesn't exist or is empty (first launch). Returns an error if the
/// file exists but can't be parsed.
pub fn load_works() -> OctostudioResult<Vec<Work>> {
    let path = works_path()?;
    load_works_from(&path)
}

pub fn load_works_from(path: &Path) -> OctostudioResult<Vec<Work>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)?;
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let works: Vec<Work> = serde_json::from_str(&text)?;
    Ok(works)
}

/// Persist the works array. Creates the parent dir if needed.
pub fn save_works(works: &[Work]) -> OctostudioResult<()> {
    let path = works_path()?;
    save_works_to(works, &path)
}

pub fn save_works_to(works: &[Work], path: &Path) -> OctostudioResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| OctostudioError::Io(e))?;
    }
    let json = serde_json::to_string_pretty(works)?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use octostudio_core::Work;
    use std::env;

    fn tmp_path(name: &str) -> std::path::PathBuf {
        env::temp_dir().join(format!("octostudio-test-{}-{}.json", std::process::id(), name))
    }

    #[test]
    fn load_missing_file_returns_empty() {
        let p = tmp_path("missing");
        let _ = std::fs::remove_file(&p);
        let works = load_works_from(&p).unwrap();
        assert!(works.is_empty());
    }

    #[test]
    fn roundtrip_preserves_a_work() {
        let p = tmp_path("roundtrip");
        let original = vec![Work::new("w-1".into())];
        save_works_to(&original, &p).unwrap();
        let loaded = load_works_from(&p).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, "w-1");
        let _ = std::fs::remove_file(&p);
    }
}
