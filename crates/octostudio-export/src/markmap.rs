//! Markmap.js.org compatible mindmap.

use std::fmt::Write as _;

use octostudio_core::PlanItem;

pub fn markmap_text(items: &[PlanItem], root: &str) -> String {
    let mut s = String::new();
    writeln!(s, "# {root}").ok();
    writeln!(s).ok();
    for it in items {
        let label = it.get("label");
        let children = it.get("children");
        writeln!(s, "## {label}").ok();
        for c in children.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            writeln!(s, "### {c}").ok();
        }
        writeln!(s).ok();
    }
    s
}
