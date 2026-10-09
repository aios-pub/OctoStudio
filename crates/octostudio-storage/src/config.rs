//! `config.json` IO — user preferences + API key.
//!
//! C4 stores a minimal config: API key + preferred theme. C5-C9 will
//! extend with model class preference, image-generation default, etc.

use std::path::Path;

use serde::{Deserialize, Serialize};

use octostudio_core::{OctostudioError, OctostudioResult};

use crate::paths::config_path;

/// Light / Dark / System — v0.6 only honours Light; C4 reserves the
/// field so the Settings screen (C9) can persist the choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl Default for ThemeMode {
    fn default() -> Self { ThemeMode::Light }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Agnes API key (sent as `Authorization: Bearer …`).
    /// None means the user hasn't entered one — the app falls back to
    /// offline demo data on any AI call.
    pub api_key: Option<String>,

    /// Theme mode (light / dark / system). v0.6 only renders light;
    /// C9 will branch on this.
    #[serde(default)]
    pub theme_mode: ThemeMode,

    /// Default model class for `text.complete` — `"fast"` (cheap) or
    /// `"strong"` (deep). Mirrors splash `class` field.
    #[serde(default = "default_model_class")]
    pub model_class: String,

    /// Whether AI illustrations are enabled by default (splash `images_on`).
    #[serde(default = "default_true")]
    pub images_on: bool,
}

fn default_model_class() -> String { "fast".to_string() }
fn default_true() -> bool { true }

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: None,
            theme_mode: ThemeMode::Light,
            model_class: default_model_class(),
            images_on: true,
        }
    }
}

impl Config {
    /// Redact the API key for display ("sk-***1234") in the Settings screen.
    pub fn masked_key(&self) -> String {
        match &self.api_key {
            Some(k) if k.len() > 8 => {
                let head = &k[..4];
                let tail = &k[k.len()-4..];
                format!("{head}…{tail}")
            }
            Some(_k) => "***".to_string(),
            None => String::new(),
        }
    }
}

pub fn load_config() -> OctostudioResult<Config> {
    let path = config_path()?;
    load_config_from(&path)
}

pub fn load_config_from(path: &Path) -> OctostudioResult<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = std::fs::read_to_string(path)?;
    if text.trim().is_empty() {
        return Ok(Config::default());
    }
    let cfg: Config = serde_json::from_str(&text)
        .map_err(|e| OctostudioError::Config(format!("config.json: {e}")))?;
    Ok(cfg)
}

pub fn save_config(cfg: &Config) -> OctostudioResult<()> {
    let path = config_path()?;
    save_config_to(cfg, &path)
}

pub fn save_config_to(cfg: &Config, path: &Path) -> OctostudioResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(cfg)?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn tmp_path(name: &str) -> std::path::PathBuf {
        env::temp_dir().join(format!("octostudio-test-{}-{}.json", std::process::id(), name))
    }

    #[test]
    fn load_missing_file_returns_default() {
        let p = tmp_path("cfg-missing");
        let _ = std::fs::remove_file(&p);
        let cfg = load_config_from(&p).unwrap();
        assert!(cfg.api_key.is_none());
        assert_eq!(cfg.theme_mode, ThemeMode::Light);
        assert!(cfg.images_on);
    }

    #[test]
    fn roundtrip_preserves_api_key() {
        let p = tmp_path("cfg-rt");
        let mut cfg = Config::default();
        cfg.api_key = Some("sk-test-1234567890".to_string());
        cfg.model_class = "strong".to_string();
        save_config_to(&cfg, &p).unwrap();
        let loaded = load_config_from(&p).unwrap();
        assert_eq!(loaded.api_key, cfg.api_key);
        assert_eq!(loaded.model_class, "strong");
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn masked_key_hides_middle() {
        let mut cfg = Config::default();
        cfg.api_key = Some("sk-1234567890abcdef".to_string());
        assert_eq!(cfg.masked_key(), "sk-1…cdef");
        cfg.api_key = Some("short".to_string());
        assert_eq!(cfg.masked_key(), "***");
        cfg.api_key = None;
        assert_eq!(cfg.masked_key(), "");
    }
}
