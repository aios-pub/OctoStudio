//! `usage.json` — daily token aggregation for the Settings "配额" panel.
//!
//! C4 keeps a per-day roll-up: each Agnes response appends one
//! [`UsageRecord`] (tokens + model + label); the file groups by date
//! ([`UsageDay`]). The Settings screen (C9) renders today's count and
//! a 7-day sparkline.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use octostudio_core::OctostudioResult;

use crate::paths::usage_path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageDay {
    pub date: String, // "YYYY-MM-DD"
    pub calls: u32,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageRecord {
    pub date: String,
    pub model: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub label: String, // e.g. "ai_gen_title" / "ask_scenario:video"
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageLog {
    /// One row per day, oldest first.
    pub days: Vec<UsageDay>,
}

pub fn load_usage() -> OctostudioResult<UsageLog> {
    let path = usage_path()?;
    load_usage_from(&path)
}

pub fn load_usage_from(path: &Path) -> OctostudioResult<UsageLog> {
    if !path.exists() {
        return Ok(UsageLog::default());
    }
    let text = std::fs::read_to_string(path)?;
    if text.trim().is_empty() {
        return Ok(UsageLog::default());
    }
    let log: UsageLog = serde_json::from_str(&text)?;
    Ok(log)
}

pub fn save_usage(log: &UsageLog) -> OctostudioResult<()> {
    let path = usage_path()?;
    save_usage_to(log, &path)
}

pub fn save_usage_to(log: &UsageLog, path: &Path) -> OctostudioResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(log)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Append one record, folding into the matching day.
pub fn record_usage(log: &mut UsageLog, rec: UsageRecord) {
    if let Some(day) = log.days.iter_mut().find(|d| d.date == rec.date) {
        day.calls += 1;
        day.prompt_tokens += rec.prompt_tokens;
        day.completion_tokens += rec.completion_tokens;
    } else {
        log.days.push(UsageDay {
            date: rec.date.clone(),
            calls: 1,
            prompt_tokens: rec.prompt_tokens,
            completion_tokens: rec.completion_tokens,
        });
    }
    // keep sorted ascending by date
    log.days.sort_by(|a, b| a.date.cmp(&b.date));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_usage_folds_into_existing_day() {
        let mut log = UsageLog::default();
        record_usage(&mut log, UsageRecord {
            date: "2026-10-09".into(),
            model: "agnes-3.0-flash".into(),
            prompt_tokens: 100,
            completion_tokens: 50,
            label: "ai_gen_title".into(),
        });
        record_usage(&mut log, UsageRecord {
            date: "2026-10-09".into(),
            model: "agnes-3.0-flash".into(),
            prompt_tokens: 200,
            completion_tokens: 80,
            label: "ai_summarize".into(),
        });
        assert_eq!(log.days.len(), 1);
        assert_eq!(log.days[0].calls, 2);
        assert_eq!(log.days[0].prompt_tokens, 300);
        assert_eq!(log.days[0].completion_tokens, 130);
    }

    #[test]
    fn record_usage_creates_new_day() {
        let mut log = UsageLog::default();
        record_usage(&mut log, UsageRecord {
            date: "2026-10-09".into(), model: "x".into(),
            prompt_tokens: 1, completion_tokens: 1, label: "x".into(),
        });
        record_usage(&mut log, UsageRecord {
            date: "2026-10-10".into(), model: "x".into(),
            prompt_tokens: 1, completion_tokens: 1, label: "x".into(),
        });
        assert_eq!(log.days.len(), 2);
        assert_eq!(log.days[0].date, "2026-10-09");
        assert_eq!(log.days[1].date, "2026-10-10");
    }
}
