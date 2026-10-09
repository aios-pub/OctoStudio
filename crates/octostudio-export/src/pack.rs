//! Video production pack: scene prompts + SRT + 合成建议 + ffmpeg 命令.

use std::fmt::Write as _;

use octostudio_core::{Composition, PlanItem};

pub fn pack_text(
    items: &[PlanItem],
    title: &str,
    logline: &str,
    comp: Option<&Composition>,
    srt: &str,
) -> String {
    let mut s = String::new();
    writeln!(s, "# 视频制作包:{title}").ok();
    writeln!(s).ok();
    writeln!(s, "> {logline}").ok();
    writeln!(s).ok();
    writeln!(s, "## 1. 逐镜 prompt").ok();
    for it in items {
        let id = it.get("id");
        let dur = it.get("duration_s");
        let shot = it.get("shot_type");
        let vp = it.get("video_prompt");
        let vo = it.get("voiceover");
        writeln!(s, "### 镜 {id}({dur}s · {shot})").ok();
        if !vo.is_empty() {
            writeln!(s, "旁白:{vo}").ok();
        }
        if !vp.is_empty() {
            writeln!(s, "prompt:`{vp}`").ok();
        }
        writeln!(s).ok();
    }
    if let Some(c) = comp {
        writeln!(s, "## 2. 合成建议").ok();
        writeln!(s, "- 转场:{}", c.transition_style).ok();
        writeln!(s, "- BGM:{}", c.bgm_mood).ok();
        writeln!(s, "- 节奏:{}", c.pacing_note).ok();
        writeln!(s, "- 调色:{}", c.color_grade).ok();
        writeln!(s, "- 字幕:{}", c.subtitle_style).ok();
        if !c.output_hint.is_empty() {
            writeln!(s, "- 输出:{}", c.output_hint).ok();
        }
        writeln!(s).ok();
    }
    writeln!(s, "## 3. SRT 字幕").ok();
    writeln!(s, "```srt").ok();
    s.push_str(srt);
    if !srt.ends_with('\n') {
        s.push('\n');
    }
    writeln!(s, "```").ok();
    writeln!(s).ok();
    writeln!(s, "## 4. ffmpeg 拼接命令").ok();
    writeln!(s, r#"```bash"#,).ok();
    writeln!(s, "# 假设每镜输出为 s1.mp4 s2.mp4 ...").ok();
    writeln!(s, "ls s*.mp4 | sort > list.txt").ok();
    writeln!(s, "ffmpeg -f concat -safe 0 -i list.txt -c copy out.mp4").ok();
    writeln!(s, "```").ok();
    s
}
