//! App state — mirrors the splash `let` bindings 1:1.
//!
//! All 35+ fields the splash keeps at the top of `main.splash` are here.
//! Defaults match the splash `let X = …` defaults so a freshly booted v0.6
//! native app behaves identically to a freshly booted card-host bundle.
//!
//! C3 only mutates `mode` (TabBar nav). C4+ wires the rest.

use std::collections::{BTreeMap, HashSet, VecDeque};

use octostudio_core::{
    Composition, ExportFormat, PlanItem, PlanKind, Screen, SortMode, Work,
};

/// Live app state. C3 keeps this single-threaded; later commits may switch
/// to a `Mutex<AppState>` if the AI polling thread needs to write back.
#[derive(Debug)]
pub struct AppState {
    pub mode: Screen,

    // works library
    pub works: Vec<Work>,
    pub seq: u64,

    // current work in edit
    pub current_id: String,
    pub current_title: String,
    pub current_source: String,
    pub current_reference: String,
    pub current_content: String,
    pub current_style: String,
    pub current_prompt: String,
    pub current_plan_title: String,
    pub current_plan_summary: String,
    pub current_plan_items: Vec<PlanItem>,

    // plan scenario
    pub plan_kind: PlanKind,
    pub plan_format: ExportFormat,
    pub current_theme: Option<String>,
    pub current_preset: Option<String>,
    pub current_composition: Option<Composition>,
    pub images_on: bool,
    pub editing_k: i64,

    // chrome
    pub status_text: String,
    pub status_detail: String,
    pub busy: bool,

    // home list
    pub search_query: String,
    pub active_tag: Option<String>,
    pub known_tags: Vec<String>,
    pub history_open_id: Option<String>,
    pub sort_mode: SortMode,
    pub select_mode: bool,
    pub selected_ids: HashSet<String>,

    // studio
    pub undo_stack: VecDeque<UndoSnapshot>,
    pub ai_panel_result: String,
    pub budget_text: String,

    // settings (C9)
    pub api_key: Option<String>,
    pub api_key_valid: bool,

    // video (C7)
    pub video_jobs: BTreeMap<String, VideoJobInfo>,
}

#[derive(Debug, Clone)]
pub struct UndoSnapshot {
    pub kind: UndoKind,
    pub value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum UndoKind {
    Title,
    Source,
    Content,
}

#[derive(Debug, Clone)]
pub struct VideoJobInfo {
    pub video_id: String,
    pub prompt: String,
    pub mode: String,
    pub status: VideoStatus,
    pub started_at: u64,
}

#[derive(Debug, Clone)]
pub enum VideoStatus {
    Queued,
    Running(u8),
    Completed(String),
    Failed(String),
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mode: Screen::Home,
            works: Vec::new(),
            seq: 0,
            current_id: String::new(),
            current_title: "未命名作品".to_string(),
            current_source: String::new(),
            current_reference: String::new(),
            current_content: String::new(),
            current_style: "改写".to_string(),
            current_prompt: String::new(),
            current_plan_title: String::new(),
            current_plan_summary: String::new(),
            current_plan_items: Vec::new(),
            plan_kind: PlanKind::None,
            plan_format: ExportFormat::Markdown,
            current_theme: None,
            current_preset: None,
            current_composition: None,
            images_on: true,
            editing_k: -1,
            status_text: "就绪".to_string(),
            status_detail: String::new(),
            busy: false,
            search_query: String::new(),
            active_tag: None,
            known_tags: Vec::new(),
            history_open_id: None,
            sort_mode: SortMode::UpdatedDesc,
            select_mode: false,
            selected_ids: HashSet::new(),
            undo_stack: VecDeque::new(),
            ai_panel_result: String::new(),
            budget_text: String::new(),
            api_key: None,
            api_key_valid: false,
            video_jobs: BTreeMap::new(),
        }
    }

    pub fn next_seq(&mut self) -> u64 {
        self.seq += 1;
        self.seq
    }
}

impl Default for AppState {
    fn default() -> Self { AppState::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_state_has_correct_defaults() {
        let s = AppState::new();
        assert_eq!(s.mode, Screen::Home);
        assert_eq!(s.current_title, "未命名作品");
        assert_eq!(s.current_style, "改写");
        assert!(s.works.is_empty());
        assert_eq!(s.editing_k, -1);
        assert!(!s.busy);
        assert!(s.images_on);
        assert!(s.selected_ids.is_empty());
    }

    #[test]
    fn next_seq_increments() {
        let mut s = AppState::new();
        assert_eq!(s.next_seq(), 1);
        assert_eq!(s.next_seq(), 2);
        assert_eq!(s.next_seq(), 3);
    }
}
