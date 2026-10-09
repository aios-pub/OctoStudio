//! Markdown renderers for 6 plan kinds.

use std::fmt::Write as _;

use octostudio_core::PlanItem;

/// `# title\n\n## summary\n\n- heading\n  body\n  ![img](prompt)\n`
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
            writeln!(s, "![{h}]({ip})").ok();
            writeln!(s).ok();
        }
    }
    s
}

/// `# title\n\n> logline\n\n| # | 时长 | 镜头 | 描述 | 旁白 |\n|...`
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

/// `# title\n\n> subtitle\n\n## slide N: kind\n- bullet\n- bullet\n\n> notes\n\n---\n\n`
pub fn rt_ppt(items: &[PlanItem], title: &str, subtitle: &str) -> String {
    let mut s = String::new();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "> {subtitle}").ok();
    writeln!(s).ok();
    for (i, it) in items.iter().enumerate() {
        let kind = it.get("kind");
        let t = it.get("title");
        let notes = it.get("notes");
        writeln!(s, "## slide {}: {kind} — {t}", i + 1).ok();
        let bullets = it.get("bullets");
        for b in bullets.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            writeln!(s, "- {b}").ok();
        }
        writeln!(s).ok();
        if !notes.is_empty() {
            writeln!(s, "> {notes}").ok();
            writeln!(s).ok();
        }
        writeln!(s, "---").ok();
        writeln!(s).ok();
    }
    s
}

/// `# title\n\n- one_liner\n\n## hook\n- pattern\n- why\n\n## structure\n- time_hint | purpose | summary\n\n## reusable\n- angle\n- skeleton\n`
pub fn rt_rows(
    items: &[PlanItem],
    title: &str,
    one_liner: &str,
    hook: Option<(&str, &str)>,
    structure: &[StructureRow],
    reusable: Option<(&str, &[String])>,
) -> String {
    let mut s = String::new();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "- {one_liner}").ok();
    writeln!(s).ok();
    if let Some((p, w)) = hook {
        writeln!(s, "## hook").ok();
        writeln!(s, "- {p}").ok();
        writeln!(s, "- {w}").ok();
        writeln!(s).ok();
    }
    if !structure.is_empty() {
        writeln!(s, "## structure").ok();
        writeln!(s, "| 时间 | 作用 | 内容 |").ok();
        writeln!(s, "|------|------|------|").ok();
        for r in structure {
            writeln!(s, "| {} | {} | {} |", r.time_hint, r.purpose, r.summary).ok();
        }
        writeln!(s).ok();
    }
    if let Some((angle, skel)) = reusable {
        writeln!(s, "## reusable").ok();
        writeln!(s, "- angle: {angle}").ok();
        writeln!(s, "- skeleton:").ok();
        for s_ in skel.iter() {
            writeln!(s, "  - {s_}").ok();
        }
        writeln!(s).ok();
    }
    for it in items {
        let label = it.get("label");
        let value = it.get("text");
        writeln!(s, "- {label}: {value}").ok();
    }
    s
}

#[derive(Debug, Clone)]
pub struct StructureRow {
    pub time_hint: String,
    pub purpose: String,
    pub summary: String,
}

/// `# title\n\n## N. style / score\ntext\n\n`
pub fn rt_titles(items: &[PlanItem]) -> String {
    let mut s = String::new();
    for (i, it) in items.iter().enumerate() {
        let t = it.get("text");
        let st = it.get("style_tag");
        let sc = it.get("score");
        writeln!(s, "## {}. {t}", i + 1).ok();
        writeln!(s, "风格: {st}  推荐指数: {sc}").ok();
        writeln!(s).ok();
    }
    s
}

/// `# title\n\n{body}\n\n标签: #tag1 #tag2 ...\n\n## 配图 prompts\n- prompt\n- prompt\n- prompt\n`
pub fn rt_xhs(items: &[PlanItem], title: &str, body: &str, tags: &[String]) -> String {
    let mut s = String::new();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "{body}").ok();
    writeln!(s).ok();
    if !tags.is_empty() {
        write!(s, "标签:").ok();
        for t in tags {
            write!(s, " #{t}").ok();
        }
        writeln!(s).ok();
        writeln!(s).ok();
    }
    writeln!(s, "## 配图 prompts").ok();
    for (i, it) in items.iter().enumerate() {
        let p = it.get("image_prompts");
        if !p.is_empty() {
            writeln!(s, "- cover {i}: {p}").ok();
        }
    }
    s
}
