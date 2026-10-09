//! Notion-flavored markdown — embeds `image_prompts` as inline code blocks
//! (Notion does not auto-render prompts, but inline code makes them
//! selectable for copy).

use std::fmt::Write as _;

use octostudio_core::PlanItem;

pub fn rt_image(items: &[PlanItem], title: &str, summary: &str) -> String {
    let mut s = String::new();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "> {summary}").ok();
    writeln!(s).ok();
    for it in items {
        let h = it.get("heading");
        let b = it.get("body");
        let ip = it.get("image_prompt");
        writeln!(s, "## {h}").ok();
        writeln!(s).ok();
        writeln!(s, "{b}").ok();
        writeln!(s).ok();
        if !ip.is_empty() {
            writeln!(s, "`{ip}`").ok();
            writeln!(s).ok();
        }
    }
    s
}

pub fn rt_video(items: &[PlanItem], title: &str, logline: &str) -> String {
    let mut s = String::new();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "> {logline}").ok();
    writeln!(s).ok();
    writeln!(s, "| # | 时长(s) | 镜头 | 描述 | 旁白 |").ok();
    writeln!(s, "|---|---------|------|------|------|").ok();
    for it in items {
        let id = it.get("id");
        let dur = it.get("duration_s");
        let shot = it.get("shot_type");
        let desc = it.get("description");
        let vo = it.get("voiceover");
        writeln!(s, "| {id} | {dur} | {shot} | {desc} | {vo} |").ok();
    }
    s
}

pub fn rt_ppt(items: &[PlanItem], title: &str, subtitle: &str) -> String {
    let mut s = String::new();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "> {subtitle}").ok();
    writeln!(s).ok();
    for (i, it) in items.iter().enumerate() {
        let kind = it.get("kind");
        let t = it.get("title");
        let bullets = it.get("bullets");
        let notes = it.get("notes");
        writeln!(s, "## {}. {kind} — {t}", i + 1).ok();
        for b in bullets.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            writeln!(s, "- {b}").ok();
        }
        if !notes.is_empty() {
            writeln!(s, "> {notes}").ok();
        }
        writeln!(s).ok();
    }
    s
}

pub fn rt_xhs(items: &[PlanItem], title: &str, body: &str, tags: &[String]) -> String {
    let mut s = String::new();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "{body}").ok();
    writeln!(s).ok();
    if !tags.is_empty() {
        write!(s, "标签:").ok();
        for t in tags {
            write!(s, " `#{t}`").ok();
        }
        writeln!(s).ok();
        writeln!(s).ok();
    }
    for (i, it) in items.iter().enumerate() {
        let p = it.get("image_prompts");
        if !p.is_empty() {
            writeln!(s, "## 配图 {i}").ok();
            writeln!(s, "`{p}`").ok();
            writeln!(s).ok();
        }
    }
    s
}
