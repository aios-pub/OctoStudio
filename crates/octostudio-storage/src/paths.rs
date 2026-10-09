//! Path resolution for `works.json` / `config.json` / `usage.json`.
//!
//! All three files live in the per-user data dir resolved by [`dirs`].
//! `ensure_data_dir()` creates the directory on first launch and is
//! idempotent.

use std::path::PathBuf;

use octostudio_core::OctostudioResult;

/// `~/Library/Application Support/octostudio/` on macOS,
/// `~/.local/share/octostudio/` on Linux,
/// `C:\Users\<user>\AppData\Roaming\octostudio\` on Windows.
pub fn data_dir() -> OctostudioResult<PathBuf> {
    let dir = dirs::data_dir()
        .ok_or_else(|| octostudio_core::OctostudioError::Config(
            "could not resolve user data dir".to_string(),
        ))?
        .join("octostudio");
    Ok(dir)
}

/// Create the data dir if it doesn't exist. Idempotent.
pub fn ensure_data_dir() -> OctostudioResult<PathBuf> {
    let dir = data_dir()?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn works_path() -> OctostudioResult<PathBuf> {
    Ok(ensure_data_dir()?.join("works.json"))
}

pub fn config_path() -> OctostudioResult<PathBuf> {
    Ok(ensure_data_dir()?.join("config.json"))
}

pub fn usage_path() -> OctostudioResult<PathBuf> {
    Ok(ensure_data_dir()?.join("usage.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_dir_ends_with_octostudio() {
        let d = data_dir().unwrap();
        assert_eq!(d.file_name().and_then(|s| s.to_str()), Some("octostudio"));
    }

    #[test]
    fn ensure_data_dir_is_idempotent() {
        let d1 = ensure_data_dir().unwrap();
        let d2 = ensure_data_dir().unwrap();
        assert_eq!(d1, d2);
        assert!(d1.is_dir());
    }
}
