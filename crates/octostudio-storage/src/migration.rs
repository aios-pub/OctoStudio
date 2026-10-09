//! One-time migration: read legacy splash `works.json` (the bundle's
//! `.local-state/works.json` from card-host) and convert to v0.6 schema.
//!
//! Splash kept two redundant fields — `plan_sections` (for `image`)
//! and `plan_scenes` (for `video`) — alongside `plan_items`. v0.6
//! makes `plan_items` the single canonical store, so the migration
//! lifts legacy entries into the unified list.
//!
//! The legacy file lives at `<bundle>/.local-state/works.json`; we
//! look for it under both the explicit bundle path and the
//! `~/.octosense/octostudio/` mirror.

use std::path::{Path, PathBuf};

use octostudio_core::{Composition, HistoryEntry, OctostudioResult, PlanItem, PlanKind, Work};

/// Read the legacy splash `works.json` and return v0.6 `Work`s. Empty
/// `Vec` if no legacy file exists or the file is malformed (the
/// migration is best-effort; on parse failure the caller falls back
/// to demo data).
pub fn migrate_from_splash() -> Vec<Work> {
    for path in candidate_paths() {
        if path.exists() {
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(legacy) = serde_json::from_str::<Vec<LegacyWork>>(&text) {
                    return legacy.into_iter().map(legacy_to_v6).collect();
                }
            }
        }
    }
    Vec::new()
}

fn candidate_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".octosense/octostudio/works.json"));
        paths.push(home.join(".local/share/octostudio/works.json"));
    }
    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join("bundle/.local-state/works.json"));
        paths.push(cwd.join("../.local-state/works.json"));
    }
    paths
}

// ---- legacy schema (splash) ----------------------------------------

#[derive(Debug, serde::Deserialize)]
struct LegacyWork {
    id: String,
    title: String,
    source: String,
    reference: String,
    content: String,
    style: String,
    prompt: String,
    plan_kind: Option<String>,
    plan_title: String,
    plan_summary: String,
    plan_items: Vec<LegacyItem>,
    plan_sections: Option<Vec<LegacyItem>>,
    plan_scenes: Option<Vec<LegacyItem>>,
    theme: Option<String>,
    composition: Option<LegacyComposition>,
    tags: Option<Vec<String>>,
    history: Option<Vec<LegacyHistory>>,
    updated: u64,
}

#[derive(Debug, serde::Deserialize)]
struct LegacyItem {
    #[serde(rename = "_k")]
    k: u64,
    #[serde(flatten)]
    fields: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, serde::Deserialize)]
struct LegacyComposition {
    transition_style: String,
    bgm_mood: String,
    pacing_note: String,
    color_grade: String,
    subtitle_style: String,
    output_hint: String,
}

#[derive(Debug, serde::Deserialize)]
struct LegacyHistory {
    prompt: String,
    result: String,
    time: u64,
}

fn legacy_to_v6(l: LegacyWork) -> Work {
    let mut plan_items: Vec<PlanItem> = Vec::with_capacity(
        l.plan_items.len()
            + l.plan_sections.as_ref().map(|v| v.len()).unwrap_or(0)
            + l.plan_scenes.as_ref().map(|v| v.len()).unwrap_or(0),
    );
    for it in l.plan_items {
        plan_items.push(PlanItem { k: it.k, fields: it.fields });
    }
    // Lift legacy `plan_sections` into plan_items (image).
    if let Some(sects) = l.plan_sections {
        for it in sects {
            plan_items.push(PlanItem { k: it.k, fields: it.fields });
        }
    }
    // Lift legacy `plan_scenes` into plan_items (video).
    if let Some(scenes) = l.plan_scenes {
        for it in scenes {
            plan_items.push(PlanItem { k: it.k, fields: it.fields });
        }
    }
    Work {
        id: l.id,
        title: l.title,
        source: l.source,
        reference: l.reference,
        content: l.content,
        style: l.style,
        prompt: l.prompt,
        plan_kind: l.plan_kind.as_deref().map(PlanKind::from_id),
        plan_title: l.plan_title,
        plan_summary: l.plan_summary,
        plan_items,
        plan_format: octostudio_core::ExportFormat::Markdown,
        theme: l.theme,
        composition: l.composition.map(|c| Composition {
            transition_style: c.transition_style,
            bgm_mood: c.bgm_mood,
            pacing_note: c.pacing_note,
            color_grade: c.color_grade,
            subtitle_style: c.subtitle_style,
            output_hint: c.output_hint,
        }),
        tags: l.tags.unwrap_or_default(),
        history: l
            .history
            .unwrap_or_default()
            .into_iter()
            .map(|h| HistoryEntry { prompt: h.prompt, result: h.result, time: h.time })
            .collect(),
        updated: l.updated,
    }
}

/// Copy legacy works into the v0.6 path. Returns the migrated works
/// (empty if no migration happened).
pub fn run_migration_to(dest: &Path) -> OctostudioResult<Vec<Work>> {
    let works = migrate_from_splash();
    if !works.is_empty() {
        let json = serde_json::to_string_pretty(&works)?;
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(dest, json)?;
    }
    Ok(works)
}
