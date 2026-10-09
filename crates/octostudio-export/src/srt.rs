//! SRT subtitle generator. `HH:MM:SS,000` time codes, one cue per scene.

use std::fmt::Write as _;

use octostudio_core::PlanItem;

pub fn srt_text(items: &[PlanItem]) -> String {
    let mut s = String::new();
    let mut t: u32 = 0;
    for (i, it) in items.iter().enumerate() {
        let dur: u32 = it.get("duration_s").parse().unwrap_or(5).max(1);
        let vo = it.get("voiceover");
        let desc = it.get("description");
        // 优先用旁白(口播视频),否则用描述(无声视频)
        let cue_text = if !vo.is_empty() { vo } else { desc };
        if cue_text.is_empty() { continue; }
        let start = t;
        let end = t + dur;
        writeln!(s, "{}", i + 1).ok();
        writeln!(s, "{} --> {}", fmt_ts(start), fmt_ts(end)).ok();
        writeln!(s, "{}", cue_text.replace('\n', " ")).ok();
        writeln!(s).ok();
        t = end;
    }
    s
}

fn pad2(n: u32) -> String {
    if n < 10 { format!("0{n}") } else { n.to_string() }
}

fn fmt_ts(secs: u32) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h}:{m}:{s},000", h = pad2(h), m = pad2(m), s = pad2(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn make(dur: u32, vo: &str) -> PlanItem {
        let mut f = BTreeMap::new();
        f.insert("duration_s".into(), dur.to_string());
        f.insert("voiceover".into(), vo.into());
        PlanItem { k: 1, fields: f }
    }

    #[test]
    fn formats_2_cues() {
        let items = vec![make(3, "hello"), make(4, "world")];
        let out = srt_text(&items);
        assert!(out.contains("00:00:00,000 --> 00:00:03,000"));
        assert!(out.contains("00:00:03,000 --> 00:00:07,000"));
        assert!(out.contains("hello"));
        assert!(out.contains("world"));
    }
}
