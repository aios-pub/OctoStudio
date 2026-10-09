//! Central router: pick the right renderer for `(plan_kind, plan_format)`
//! and run it against the current [`Work`].

use octostudio_core::{Composition, ExportFormat, PlanItem, PlanKind, Work};

use crate::markdown;
use crate::markmap;
use crate::marp;
use crate::pack;
use crate::srt;
use crate::wechat;

pub fn render_plan_as_text(work: &Work) -> String {
    let items = &work.plan_items;
    let title = if work.plan_title.is_empty() {
        &work.title
    } else {
        &work.plan_title
    };
    let summary = &work.plan_summary;
    let kind = work.plan_kind.unwrap_or(PlanKind::None);

    match (work.plan_format, kind) {
        (ExportFormat::Srt, PlanKind::Video) => srt::srt_text(items),
        (ExportFormat::Pack, PlanKind::Video) => pack::pack_text(
            items,
            title,
            summary,
            work.composition.as_ref(),
            &srt::srt_text(items),
        ),
        (ExportFormat::Marp, _) if kind == PlanKind::Ppt => marp::marp_text(items, title, summary),
        (ExportFormat::Markmap, _) if kind == PlanKind::Mindmap => {
            let root = items.first().map(|i| i.get("root")).unwrap_or("主题").to_string();
            markmap::markmap_text(items, &root)
        }
        (fmt, kind) => render_by_kind(items, title, summary, fmt, kind),
    }
}

fn render_by_kind(
    items: &[PlanItem],
    title: &str,
    summary: &str,
    fmt: ExportFormat,
    kind: PlanKind,
) -> String {
    match (fmt, kind) {
        (ExportFormat::Markdown, PlanKind::Image) => markdown::rt_image(items, title, summary),
        (ExportFormat::Markdown, PlanKind::Video) => markdown::rt_video(items, title, summary),
        (ExportFormat::Markdown, PlanKind::Ppt) => markdown::rt_ppt(items, title, summary),
        (ExportFormat::Markdown, PlanKind::Titles) => markdown::rt_titles(items),
        (ExportFormat::Markdown, PlanKind::Xhs) => {
            // 拆 tags + body from items
            let tags: Vec<String> = items.iter().map(|i| i.get("tags").to_string()).filter(|s| !s.is_empty()).collect();
            let body_item = items.first();
            let body = body_item.map(|i| i.get("body").to_string()).unwrap_or_default();
            markdown::rt_xhs(items, title, &body, &tags)
        }
        (ExportFormat::Markdown, _) => {
            // 通用 rows(teardown / script / mindmap / quotes / none)
            markdown::rt_rows(items, title, summary, None, &[], None)
        }
        (ExportFormat::Wechat, PlanKind::Image) => wechat::rt_image(items, title, summary),
        (ExportFormat::Wechat, PlanKind::Video) => wechat::rt_video(items, title, summary),
        (ExportFormat::Wechat, PlanKind::Ppt) => wechat::rt_ppt(items, title, summary),
        (ExportFormat::Wechat, PlanKind::Titles) => wechat::rt_titles(items),
        (ExportFormat::Wechat, PlanKind::Xhs) => {
            let tags: Vec<String> = items.iter().map(|i| i.get("tags").to_string()).filter(|s| !s.is_empty()).collect();
            let body_item = items.first();
            let body = body_item.map(|i| i.get("body").to_string()).unwrap_or_default();
            wechat::rt_xhs(items, title, &body, &tags)
        }
        (ExportFormat::Wechat, _) => {
            wechat::rt_rows(items, title, summary, None)
        }
        (ExportFormat::Notion, PlanKind::Image) => notion::rt_image(items, title, summary),
        (ExportFormat::Notion, PlanKind::Video) => notion::rt_video(items, title, summary),
        (ExportFormat::Notion, PlanKind::Ppt) => notion::rt_ppt(items, title, summary),
        (ExportFormat::Notion, PlanKind::Xhs) => {
            let tags: Vec<String> = items.iter().map(|i| i.get("tags").to_string()).filter(|s| !s.is_empty()).collect();
            let body_item = items.first();
            let body = body_item.map(|i| i.get("body").to_string()).unwrap_or_default();
            notion::rt_xhs(items, title, &body, &tags)
        }
        (ExportFormat::Notion, _) => {
            // Notion fallback to markdown
            markdown::rt_rows(items, title, summary, None, &[], None)
        }
        // Srt / Pack / Marp / Markmap only meaningful for specific kinds;
        // for any other kind, fall back to a plain summary.
        (ExportFormat::Srt, _) | (ExportFormat::Pack, _) | (ExportFormat::Marp, _) | (ExportFormat::Markmap, _) => {
            // Special-case dispatch was already done at top of render_plan_as_text.
            // Reaching here means the (fmt, kind) wasn't matched earlier —
            // emit a header with the work title.
            format!("# {title}\n\n> {summary}\n")
        }
    }
}

mod notion {
    pub use crate::notion::*;
}
