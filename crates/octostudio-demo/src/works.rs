//! Two demo works, shown on first launch when the user has no API key
//! and no legacy splash data.
//!
//! Names mirror the splash demo (`测试作品一` / `测试作品二`) so the
//! visual is consistent across both versions.

use std::collections::BTreeMap;

use octostudio_core::{ExportFormat, PlanItem, PlanKind, Work};

/// Build the first-launch demo works list. Returns a `Vec<Work>` with
/// exactly 2 entries (a 视频分镜 example and a 图文文章 example).
pub fn demo_works() -> Vec<Work> {
    vec![demo_work_video(), demo_work_image()]
}

fn demo_work_video() -> Work {
    let mut fields = BTreeMap::new();
    fields.insert("id".into(), "S1".into());
    fields.insert("duration_s".into(), "5".into());
    fields.insert("shot_type".into(), "特写".into());
    fields.insert("description".into(),
        "海浪轻拍礁石,镜头贴近水面,晨光从云缝透出,营造静谧氛围。".into());
    fields.insert("voiceover".into(),
        "「清晨四点五十分,海面还蒙着夜的薄纱。」".into());
    fields.insert("video_prompt".into(),
        "cinematic coastal sunrise, close-up wave lapping reef rock, golden hour, soft light".into());

    let mut fields2 = BTreeMap::new();
    fields2.insert("id".into(), "S2".into());
    fields2.insert("duration_s".into(), "6".into());
    fields2.insert("shot_type".into(), "中景".into());
    fields2.insert("description".into(),
        "主角站在礁石上望向远方,海风吹动头发,画面沉稳大气。".into());
    fields2.insert("voiceover".into(),
        "「我常在同一个地方醒来,在不同的地方出发。」".into());
    fields2.insert("video_prompt".into(),
        "cinematic medium shot, woman standing on coastal rock, ocean wind, golden hour, contemplative".into());

    let mut fields3 = BTreeMap::new();
    fields3.insert("id".into(), "S3".into());
    fields3.insert("duration_s".into(), "5".into());
    fields3.insert("shot_type".into(), "全景".into());
    fields3.insert("description".into(),
        "无人机拉升,俯瞰整片海岸线,日出的金色铺满海面。".into());
    fields3.insert("voiceover".into(),
        "「海与天的边界,原来是我给自己的边界。」".into());
    fields3.insert("video_prompt".into(),
        "aerial drone pull-up, wide coastline sunrise panorama, golden ocean".into());

    Work {
        id: "w-demo-1".into(),
        title: "测试作品一(视频分镜 demo)".into(),
        source: "夏日海边慢生活散文".into(),
        reference: String::new(),
        content: "清晨四点五十分,海面还蒙着夜的薄纱。\n我常在同一个地方醒来,在不同的地方出发。\n海与天的边界,原来是我给自己的边界。".into(),
        style: "改写".into(),
        prompt: "夏日海边慢生活,温暖散文风".into(),
        plan_kind: Some(PlanKind::Video),
        plan_format: ExportFormat::Markdown,
        plan_title: "夏日海边慢生活".into(),
        plan_summary: "3 镜分镜 + AI 合成建议 + SRT 字幕 + 视频制作包".into(),
        plan_items: vec![
            PlanItem { k: 1, fields },
            PlanItem { k: 2, fields: fields2 },
            PlanItem { k: 3, fields: fields3 },
        ],
        theme: None,
        composition: Some(octostudio_core::Composition {
            transition_style: "淡入淡出 + 缓慢推拉".into(),
            bgm_mood: "钢琴 + 海浪白噪音".into(),
            pacing_note: "慢节奏,前 1/3 静默,中段起 BGM".into(),
            color_grade: "暖橙 + 青蓝(电影感)".into(),
            subtitle_style: "底部居中,白色描边,微软雅黑".into(),
            output_hint: "1080P 16:9 · 16s · ffmpeg concat demuxer".into(),
        }),
        tags: vec!["demo".into(), "vlog".into(), "海边".into()],
        history: Vec::new(),
        updated: 1,
    }
}

fn demo_work_image() -> Work {
    let mut fields1 = BTreeMap::new();
    fields1.insert("heading".into(), "清晨四点的海".into());
    fields1.insert("body".into(),
        "海面还蒙着夜的薄纱,礁石上覆一层薄霜。远处灯塔的光转了一圈又一圈,像某个不肯睡的人在反复确认时间。".into());
    fields1.insert("image_prompt".into(),
        "misty coastal pre-dawn, lighthouse beam sweeping over dark water, cinematic, photographic".into());

    let mut fields2 = BTreeMap::new();
    fields2.insert("heading".into(), "六点的风".into());
    fields2.insert("body".into(),
        "太阳从云缝里探出第一道光,海风裹着盐味冲进窗,所有睡不着的念头都被吹散了。".into());
    fields2.insert("image_prompt".into(),
        "early morning sun rays through clouds, sea breeze blowing curtains, warm light".into());

    let mut fields3 = BTreeMap::new();
    fields3.insert("heading".into(), "九点的餐桌".into());
    fields3.insert("body".into(),
        "早餐是一碗白粥、一碟咸鸭蛋、一只刚从树上摘下的青芒。生活的奢侈,不在价格,在刚好。".into());
    fields3.insert("image_prompt".into(),
        "minimalist breakfast table, white porridge bowl, salted duck egg, fresh green mango, soft natural light".into());

    Work {
        id: "w-demo-2".into(),
        title: "测试作品二(图文文章 demo)".into(),
        source: "夏日海边慢生活长文".into(),
        reference: String::new(),
        content:
            "清晨四点五十分,海面还蒙着夜的薄纱。\n\
             六点,太阳从云缝里探出第一道光。\n\
             九点,早餐是一碗白粥、一碟咸鸭蛋、一只青芒。\n\
             生活的奢侈,不在价格,在刚好。".into(),
        style: "改写".into(),
        prompt: "夏日海边慢生活,温暖散文风".into(),
        plan_kind: Some(PlanKind::Image),
        plan_format: ExportFormat::Markdown,
        plan_title: "夏日海边慢生活图文".into(),
        plan_summary: "3 段图文 + 配图 prompt 列表".into(),
        plan_items: vec![
            PlanItem { k: 1, fields: fields1 },
            PlanItem { k: 2, fields: fields2 },
            PlanItem { k: 3, fields: fields3 },
        ],
        theme: Some("prose".into()),
        composition: None,
        tags: vec!["demo".into(), "图文".into(), "散文".into()],
        history: Vec::new(),
        updated: 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_works_returns_two() {
        let w = demo_works();
        assert_eq!(w.len(), 2);
        assert!(w.iter().any(|x| x.plan_kind == Some(octostudio_core::PlanKind::Video)));
        assert!(w.iter().any(|x| x.plan_kind == Some(octostudio_core::PlanKind::Image)));
    }
}
