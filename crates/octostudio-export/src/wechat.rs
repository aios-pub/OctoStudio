//! 公众号 / 小红书 plain-text variants — no Markdown syntax, paste-friendly.

use std::fmt::Write as _;

use octostudio_core::PlanItem;

pub fn rt_image(items: &[PlanItem], title: &str, summary: &str) -> String {
    let mut s = String::new();
    writeln!(s, "【{title}】").ok();
    writeln!(s).ok();
    writeln!(s, "{summary}").ok();
    writeln!(s).ok();
    for it in items {
        let h = it.get("heading");
        let b = it.get("body");
        writeln!(s, "▍{h}").ok();
        writeln!(s, "{b}").ok();
        writeln!(s).ok();
    }
    s
}

pub fn rt_video(items: &[PlanItem], title: &str, logline: &str) -> String {
    let mut s = String::new();
    writeln!(s, "【{title}】").ok();
    writeln!(s).ok();
    writeln!(s, "{logline}").ok();
    writeln!(s).ok();
    for it in items {
        let id = it.get("id");
        let dur = it.get("duration_s");
        let shot = it.get("shot_type");
        let desc = it.get("description");
        let vo = it.get("voiceover");
        writeln!(s, "▍镜 {id}({dur}s · {shot})").ok();
        writeln!(s, "{desc}").ok();
        writeln!(s).ok();
        if !vo.is_empty() {
            writeln!(s, "「{vo}」").ok();
            writeln!(s).ok();
        }
    }
    s
}

pub fn rt_ppt(items: &[PlanItem], title: &str, subtitle: &str) -> String {
    let mut s = String::new();
    writeln!(s, "【{title} — {subtitle}】").ok();
    writeln!(s).ok();
    for (i, it) in items.iter().enumerate() {
        let t = it.get("title");
        let bullets = it.get("bullets");
        let notes = it.get("notes");
        writeln!(s, "▍{}. {t}", i + 1).ok();
        for b in bullets.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            writeln!(s, "  · {b}").ok();
        }
        if !notes.is_empty() {
            writeln!(s, "  备注:{notes}").ok();
        }
        writeln!(s).ok();
    }
    s
}

pub fn rt_rows(
    items: &[PlanItem],
    title: &str,
    one_liner: &str,
    hook: Option<(&str, &str)>,
) -> String {
    let mut s = String::new();
    writeln!(s, "【{title}】").ok();
    writeln!(s).ok();
    writeln!(s, "{one_liner}").ok();
    writeln!(s).ok();
    if let Some((p, w)) = hook {
        writeln!(s, "▍钩子:{p}").ok();
        writeln!(s, "{w}").ok();
        writeln!(s).ok();
    }
    for it in items {
        let label = it.get("label");
        let value = it.get("text");
        writeln!(s, "▍{label}:{value}").ok();
    }
    s
}

pub fn rt_titles(items: &[PlanItem]) -> String {
    let mut s = String::new();
    for (i, it) in items.iter().enumerate() {
        let t = it.get("text");
        let st = it.get("style_tag");
        let sc = it.get("score");
        writeln!(s, "{}. 【{}】 ({}/100 · {})", i + 1, t, sc, st).ok();
    }
    s
}

pub fn rt_xhs(items: &[PlanItem], title: &str, body: &str, tags: &[String]) -> String {
    let mut s = String::new();
    writeln!(s, "✨ {title} ✨").ok();
    writeln!(s).ok();
    writeln!(s, "{body}").ok();
    writeln!(s).ok();
    if !tags.is_empty() {
        for t in tags {
            writeln!(s, "#{t}").ok();
        }
        writeln!(s).ok();
    }
    writeln!(s, "📸 配图建议:").ok();
    for (i, it) in items.iter().enumerate() {
        let p = it.get("image_prompts");
        if !p.is_empty() {
            writeln!(s, "{i}. {p}").ok();
        }
    }
    s
}
