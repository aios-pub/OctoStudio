//! Marp frontmatter + per-slide bullets + notes (as HTML comment).

use std::fmt::Write as _;

use octostudio_core::PlanItem;

pub fn marp_text(items: &[PlanItem], title: &str, subtitle: &str) -> String {
    let mut s = String::new();
    writeln!(s, "---").ok();
    writeln!(s, "marp: true").ok();
    writeln!(s, "theme: default").ok();
    writeln!(s, "title: {title}").ok();
    writeln!(s, "subtitle: {subtitle}").ok();
    writeln!(s, "---").ok();
    writeln!(s).ok();
    writeln!(s, "# {title}").ok();
    writeln!(s).ok();
    writeln!(s, "## {subtitle}").ok();
    writeln!(s).ok();
    for (i, it) in items.iter().enumerate() {
        let t = it.get("title");
        let kind = it.get("kind");
        let bullets = it.get("bullets");
        let notes = it.get("notes");
        writeln!(s, "---").ok();
        writeln!(s).ok();
        writeln!(s, "# {}. {kind} — {t}", i + 1).ok();
        writeln!(s).ok();
        for b in bullets.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            writeln!(s, "- {b}").ok();
        }
        if !notes.is_empty() {
            writeln!(s).ok();
            writeln!(s, "<!-- notes: {notes} -->").ok();
        }
    }
    s
}
