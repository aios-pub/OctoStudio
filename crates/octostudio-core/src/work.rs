//! Domain data: [`Work`], [`PlanItem`], [`Composition`], [`HistoryEntry`].
//!
//! These match the splash `works.json` schema exactly so that the migration
//! in `octostudio-storage` can read the legacy `<bundle>/.local-state/works.json`
//! without lossy mapping. The legacy `plan_sections` / `plan_scenes` mirror
//! fields are intentionally dropped in v0.6 — `plan_items` is canonical.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::enums::PlanKind;

/// A single works.json entry. Persisted across launches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub id: String,
    pub title: String,
    pub source: String,
    pub reference: String,
    pub content: String,
    pub style: String,
    pub prompt: String,
    pub plan_kind: Option<PlanKind>,
    pub plan_title: String,
    pub plan_summary: String,
    pub plan_items: Vec<PlanItem>,
    pub theme: Option<String>,
    pub composition: Option<Composition>,
    pub tags: Vec<String>,
    pub history: Vec<HistoryEntry>,
    pub updated: u64,
}

impl Default for Work {
    fn default() -> Self {
        Work {
            id: String::new(),
            title: "未命名作品".to_string(),
            source: String::new(),
            reference: String::new(),
            content: String::new(),
            style: "改写".to_string(),
            prompt: String::new(),
            plan_kind: None,
            plan_title: String::new(),
            plan_summary: String::new(),
            plan_items: Vec::new(),
            theme: None,
            composition: None,
            tags: Vec::new(),
            history: Vec::new(),
            updated: 0,
        }
    }
}

impl Work {
    pub fn new(id: String) -> Self {
        Work { id, ..Work::default() }
    }

    pub fn display_title(&self) -> &str {
        if self.title.is_empty() { "未命名作品" } else { &self.title }
    }

    pub fn kind_emoji(&self) -> &'static str {
        self.plan_kind.unwrap_or(PlanKind::None).emoji()
    }

    pub fn kind_label(&self) -> String {
        let k = self.plan_kind.unwrap_or(PlanKind::None);
        if matches!(k, PlanKind::None) {
            if self.style.is_empty() { "文章".to_string() } else { self.style.clone() }
        } else {
            k.short().to_string()
        }
    }
}

/// One row in a [`Work::plan_items`] list.
///
/// `_k` is the splash stable key (sequence number at creation) — used by
/// edit/move/delete operations so the list can be reordered without losing
/// the user's selection. `fields` is the scenario-specific payload
/// (e.g. video scene: { duration_s, shot_type, description, voiceover, video_prompt }).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItem {
    #[serde(rename = "_k")]
    pub k: u64,
    #[serde(flatten)]
    pub fields: BTreeMap<String, String>,
}

impl PlanItem {
    pub fn new(k: u64) -> Self {
        PlanItem { k, fields: BTreeMap::new() }
    }

    pub fn get(&self, key: &str) -> &str {
        self.fields.get(key).map(String::as_str).unwrap_or("")
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) {
        self.fields.insert(key.to_string(), value.into());
    }
}

/// Video composition suggestion (returned by AI 合成建议 button).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Composition {
    pub transition_style: String,
    pub bgm_mood: String,
    pub pacing_note: String,
    pub color_grade: String,
    pub subtitle_style: String,
    pub output_hint: String,
}

impl Default for Composition {
    fn default() -> Self {
        Composition {
            transition_style: String::new(),
            bgm_mood: String::new(),
            pacing_note: String::new(),
            color_grade: String::new(),
            subtitle_style: String::new(),
            output_hint: String::new(),
        }
    }
}

/// One AI history row, max 3 per work (capped on save).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub prompt: String,
    pub result: String,
    pub time: u64,
}

impl HistoryEntry {
    pub fn new(prompt: String, result: String, time: u64) -> Self {
        HistoryEntry { prompt, result, time }
    }
}
