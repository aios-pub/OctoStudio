//! 9 plan_kind demo datasets, ported 1:1 from the splash
//! `demo_article_sections` / `demo_video_scenes` / `demo_ppt_slides` /
//! `demo_teardown_items` / `demo_titles_items` / `demo_xhs_items` /
//! `demo_script_items` / `demo_mindmap_items` / `demo_quotes_items` /
//! `demo_composition` functions on `v0.5-alpha-ui-polish`.
//!
//! Used as the no-API-key fallback for `ask_scenario()` / `ask_composition()`.
//! Field names match the splash schema verbatim so [`crate::plans::load_demo_plan`]
//! can drop in for the real Agnes result without UI changes.
//!
//! Top-level dispatch [`load_demo_plan(kind)`] returns
//! `(plan_title, plan_summary, plan_items, composition)` so the caller
//! can hydrate [`octostudio_core::Work`] the same way splash's
//! `load_demo_plan()` did.

use std::collections::BTreeMap;

use octostudio_core::{Composition, PlanItem, PlanKind};

/// Build a `PlanItem` with a stable `k` (splash used a counter; here
/// we hash the index to keep the function pure).
fn item(k: u64, fields: &[(&str, &str)]) -> PlanItem {
    let mut m = BTreeMap::new();
    for (k_, v) in fields {
        m.insert((*k_).to_string(), (*v).to_string());
    }
    PlanItem { k, fields: m }
}

// ---------- demo_article_sections ----------------------------------------

pub fn demo_article_sections() -> (String, Vec<PlanItem>) {
    let title = "夏日海边慢生活".to_string();
    let items = vec![
        item(1, &[
            ("heading", "海浪轻拂"),
            ("body", "清晨六点的海滩,只有浪花与风的对话。脚下的细沙还带着夜的凉意,远方有渔船在薄雾里慢慢靠岸。"),
            ("image_prompt", "a serene beach at dawn, soft golden light, gentle waves lapping, minimalist style, warm tones"),
        ]),
        item(2, &[
            ("heading", "椰影与凉茶"),
            ("body", "树荫下的吊床轻轻摇晃,一杯冰椰,时间似乎也放慢了脚步。海风从窗纱里挤进来,带来远处浪花的节拍。"),
            ("image_prompt", "a hammock under coconut trees, tropical iced tea with condensation, soft dappled sunlight, slow life mood"),
        ]),
        item(3, &[
            ("heading", "落日告别"),
            ("body", "太阳慢慢沉入海平线,天空被染成温柔的紫与橘。海风带走一天的喧嚣,只剩下潮汐的低语。"),
            ("image_prompt", "sunset over calm sea, purple and orange sky gradient, peaceful horizon, cinematic warm light"),
        ]),
        item(4, &[
            ("heading", "星空夜话"),
            ("body", "夜深了,海风轻拂过听浪的人。银河缓缓铺开,星星落在浪尖上,又被浪卷走。"),
            ("image_prompt", "starry night beach, calm sea, milky way overhead, moonlight reflecting on water, peaceful mood"),
        ]),
    ];
    (title, items)
}

// ---------- demo_video_scenes --------------------------------------------

pub fn demo_video_scenes() -> (String, Vec<PlanItem>) {
    let title = "夏日海边慢生活".to_string();
    let items = vec![
        item(1, &[
            ("id", "S1"), ("duration_s", "5"), ("shot_type", "全景航拍"),
            ("description", "无人机从远海掠过海岸线,浪花轻拂金色沙滩。"),
            ("voiceover", "这是一个关于夏日海边慢生活的故事,从清晨的第一缕光开始。"),
            ("video_prompt", "aerial drone shot of a calm coastline at golden hour, slow motion waves, soft warm light, cinematic 4k"),
        ]),
        item(2, &[
            ("id", "S2"), ("duration_s", "8"), ("shot_type", "中景跟拍"),
            ("description", "主角赤脚走在沙滩上,留下浅浅的脚印。"),
            ("voiceover", "脚下的细沙还带着夜的凉意,远方有渔船在薄雾里慢慢靠岸。"),
            ("video_prompt", "medium tracking shot, person walking barefoot on warm sand, gentle waves, soft natural light, slow motion"),
        ]),
        item(3, &[
            ("id", "S3"), ("duration_s", "6"), ("shot_type", "特写静物"),
            ("description", "一杯冰椰的特写,水珠沿着杯壁滑下。"),
            ("voiceover", "树荫下的吊床轻轻摇晃,一杯冰椰,时间似乎也放慢了脚步。"),
            ("video_prompt", "close-up of tropical iced coconut drink with ice cubes, condensation on glass, soft dappled light, shallow depth of field"),
        ]),
        item(4, &[
            ("id", "S4"), ("duration_s", "10"), ("shot_type", "全景延时"),
            ("description", "落日延时,天空从金黄变成紫蓝。"),
            ("voiceover", "太阳慢慢沉入海平线,天空被染成温柔的紫与橘。"),
            ("video_prompt", "time-lapse sunset over calm sea, golden hour to purple twilight, cinematic color grading"),
        ]),
    ];
    (title, items)
}

// ---------- demo_ppt_slides ----------------------------------------------

pub fn demo_ppt_slides() -> (String, Vec<PlanItem>) {
    let title = "夏日海边慢生活".to_string();
    let items = vec![
        item(1, &[
            ("kind", "cover"),
            ("title", "夏日海边慢生活"),
            ("bullets", "一场关于节奏的实验;从清晨到星夜"),
            ("notes", "开场问候,一句话点题:慢不是懒,是把时间花在感觉上。"),
            ("visual_prompt", "minimal cover, calm sea horizon at dawn, warm cream background, soft typography space"),
        ]),
        item(2, &[
            ("kind", "agenda"),
            ("title", "今天聊什么"),
            ("bullets", "清晨:浪与光;白天:椰影与凉茶;黄昏:落日仪式;夜晚:星空夜话"),
            ("notes", "20 秒过一遍结构,不展开。"),
            ("visual_prompt", "simple agenda illustration, four small line-art icons of sun, coconut, sunset, stars, minimal style"),
        ]),
        item(3, &[
            ("kind", "content"),
            ("title", "清晨 · 海浪轻拂"),
            ("bullets", "六点的海滩只有浪与风;细沙带着夜的凉意;渔船在薄雾里靠岸"),
            ("notes", "讲感官细节,放慢语速。"),
            ("visual_prompt", "serene beach at dawn, soft golden light, gentle waves lapping, minimalist warm tones"),
        ]),
        item(4, &[
            ("kind", "content"),
            ("title", "白天 · 椰影与凉茶"),
            ("bullets", "吊床、冰椰与窗纱里的海风;把午后的时间折进树荫"),
            ("notes", "举一个'什么都不做'的例子。"),
            ("visual_prompt", "hammock under coconut trees, iced coconut drink, dappled sunlight, slow life mood"),
        ]),
        item(5, &[
            ("kind", "content"),
            ("title", "黄昏 · 落日仪式"),
            ("bullets", "提前十分钟等一场日落;看天空从金黄烧到紫"),
            ("notes", "停顿,让画面说话。"),
            ("visual_prompt", "sunset over calm sea, orange to purple gradient sky, silhouette of quiet shore"),
        ]),
        item(6, &[
            ("kind", "end"),
            ("title", "慢下来,看得见"),
            ("bullets", "谢谢观看;愿你有海风与时间"),
            ("notes", "收尾金句,停两秒再谢幕。"),
            ("visual_prompt", "minimal ending slide, starry beach silhouette, calm navy palette"),
        ]),
    ];
    (title, items)
}

// ---------- demo_teardown_items ------------------------------------------

pub fn demo_teardown_items() -> (String, Vec<PlanItem>) {
    let title = "夏日海边慢生活 爆款拆解".to_string();
    let items = vec![
        item(1, &[("label", "一句话"), ("text", "用 3 分钟把'慢生活'拆成可执行的感官清单,治愈系节奏。")]),
        item(2, &[("label", "钩子 · 反差开场"), ("text", "开头用'你有多久没看完一场日落'制造停顿,直接锁定情绪。")]),
        item(3, &[("label", "结构 · 00:00 开场"), ("text", "抛出问题+海边空镜,建立治愈预期。")]),
        item(4, &[("label", "结构 · 00:20 分段"), ("text", "按清晨/白天/黄昏/夜晚四段推进,每段一个画面+一个建议。")]),
        item(5, &[("label", "结构 · 02:30 收尾"), ("text", "回扣开头问题,给出'今晚就看一场日落'的行动号召。")]),
        item(6, &[("label", "镜头 · 固定空镜"), ("text", "情绪铺垫,低成本高氛围。")]),
        item(7, &[("label", "镜头 · 慢推特写"), ("text", "关键道具(冰椰/吊床)慢推,强化质感与呼吸感。")]),
        item(8, &[("label", "金句"), ("text", "慢不是懒,是把时间花在感觉上。")]),
        item(9, &[("label", "复用角度"), ("text", "任何'生活方式'主题都可套用:一个提问 + 四个时段 + 一个行动号召。")]),
        item(10, &[("label", "复用骨架"), ("text", "钩子:你有多久没有…?")]),
        item(11, &[("label", "复用骨架"), ("text", "分时段展开:清晨/白天/黄昏/夜晚各一个画面+一个建议")]),
        item(12, &[("label", "复用骨架"), ("text", "收尾:回扣提问,给出今晚就能做的一小步")]),
        item(13, &[("label", "要点"), ("text", "字幕与配音同频,信息密度低一点更治愈。")]),
    ];
    (title, items)
}

// ---------- demo_titles_items --------------------------------------------

pub fn demo_titles_items() -> (String, Vec<PlanItem>) {
    let title = "标题候选".to_string();
    let items = vec![
        item(1, &[("text", "你有多久没看完一场日落了"), ("style_tag", "悬念"), ("score", "92")]),
        item(2, &[("text", "夏日海边慢生活指南:从清晨到星夜"), ("style_tag", "干货"), ("score", "88")]),
        item(3, &[("text", "把三天假期过成一个月的 4 种海边节奏"), ("style_tag", "数字"), ("score", "85")]),
        item(4, &[("text", "不是逃避生活,是去找回节奏"), ("style_tag", "情感"), ("score", "83")]),
        item(5, &[("text", "海风、冰椰与落日:一份感官慢生活清单"), ("style_tag", "清单"), ("score", "90")]),
        item(6, &[("text", "在海边,我重新学会了'无所事事'"), ("style_tag", "第一人称"), ("score", "86")]),
        item(7, &[("text", "慢下来,是今年最划算的投资"), ("style_tag", "反差"), ("score", "81")]),
        item(8, &[("text", "清晨六点的海滩,治好了我的精神内耗"), ("style_tag", "情绪"), ("score", "89")]),
    ];
    (title, items)
}

// ---------- demo_xhs_items -----------------------------------------------

pub fn demo_xhs_items() -> (String, Vec<PlanItem>) {
    let title = "夏日海边慢生活".to_string();
    let items = vec![
        item(1, &[("heading", "配图 1 · 封面"), ("body", "girl standing on calm beach at golden hour, film photography style, warm tones")]),
        item(2, &[("heading", "配图 2 · 白天"), ("body", "hammock under coconut trees with iced coconut drink, dappled sunlight, cozy slow life")]),
        item(3, &[("heading", "配图 3 · 夜晚"), ("body", "starry night beach with milky way, warm lantern light, peaceful mood")]),
    ];
    (title, items)
}

// ---------- demo_script_items --------------------------------------------

pub fn demo_script_items() -> (String, Vec<PlanItem>) {
    let title = "口播稿 · 夏日海边慢生活".to_string();
    let items = vec![
        item(1, &[("label", "钩子 · 前3秒"), ("text", "你有多久,没看完一场完整的日落了?")]),
        item(2, &[("label", "节拍 · 痛点"), ("text", "我们总说没时间,其实是把时间切碎了。")]),
        item(3, &[("label", "节拍 · 场景"), ("text", "清晨六点的海滩,只有浪和风在说话。")]),
        item(4, &[("label", "节拍 · 方法"), ("text", "慢生活不用辞职,从一场日落开始就好。")]),
        item(5, &[("label", "节拍 · 升华"), ("text", "慢不是懒,是把时间花在感觉上。")]),
        item(6, &[("label", "CTA"), ("text", "今晚,去看一场日落吧。关注我,一起把日子过慢一点。")]),
    ];
    (title, items)
}

// ---------- demo_mindmap_items -------------------------------------------

pub fn demo_mindmap_items() -> (String, Vec<PlanItem>) {
    let title = "夏日海边慢生活".to_string();
    let items = vec![
        item(1, &[("label", "感官"), ("text", "海浪声;椰影;冰椰的凉;日落的光")]),
        item(2, &[("label", "节奏"), ("text", "清晨散步;午后吊床;黄昏看日落;夜里数星星")]),
        item(3, &[("label", "装备"), ("text", "草帽;帆布包;胶片相机;防晒")]),
        item(4, &[("label", "心态"), ("text", "不赶行程;允许无所事事;记录瞬间")]),
    ];
    (title, items)
}

// ---------- demo_quotes_items --------------------------------------------

pub fn demo_quotes_items() -> (String, Vec<PlanItem>) {
    let title = "金句语录".to_string();
    let items = vec![
        item(1, &[("label", "开场"), ("text", "海浪不着急,所以每天都好看。")]),
        item(2, &[("label", "点题"), ("text", "慢不是懒,是把时间花在感觉上。")]),
        item(3, &[("label", "转折"), ("text", "把日程表倒过来,先填进一场日落。")]),
        item(4, &[("label", "反转"), ("text", "无所事事,是感官的加班。")]),
        item(5, &[("label", "结尾"), ("text", "风住的时候,海会替你说话。")]),
        item(6, &[("label", "升华"), ("text", "假期不是逃离,是回来。")]),
        item(7, &[("label", "结尾"), ("text", "看得越慢,记得越深。")]),
        item(8, &[("label", "开场"), ("text", "生活的带宽,要留给潮汐。")]),
    ];
    (title, items)
}

// ---------- demo_composition ---------------------------------------------

pub fn demo_composition() -> Composition {
    Composition {
        transition_style: "柔和叠化,段落间 0.5s 白闪".into(),
        bgm_mood: "Lo-fi 钢琴 + 海浪白噪音,BPM 70".into(),
        pacing_note: "前 10 秒钩子,之后每镜 5-8 秒,结尾留 2 秒空镜".into(),
        color_grade: "暖调奶油色,高光偏橙,阴影偏青".into(),
        subtitle_style: "居中底部,白字细黑边,每镜一句".into(),
        output_hint: "1080p · 24fps · 竖屏裁切注意安全区".into(),
    }
}

// ---------- top-level dispatch (mirrors splash load_demo_plan) -----------

/// No-API-key fallback for `ask_scenario()`. Returns the same shape that
/// [`crate::plans::load_demo_plan`] did in splash:
/// `(plan_title, plan_summary, plan_items, composition)`.
pub fn load_demo_plan(kind: PlanKind) -> (String, String, Vec<PlanItem>, Option<Composition>) {
    use PlanKind::*;
    let (title, items) = match kind {
        None     => demo_article_sections(),
        Image    => demo_article_sections(),
        Video    => demo_video_scenes(),
        Ppt      => demo_ppt_slides(),
        Teardown => demo_teardown_items(),
        Titles   => demo_titles_items(),
        Xhs      => demo_xhs_items(),
        Script   => demo_script_items(),
        Mindmap  => demo_mindmap_items(),
        Quotes   => demo_quotes_items(),
    };
    let summary = summary_for(kind, &items);
    let comp: Option<Composition> = if kind == PlanKind::Video {
        Some(demo_composition())
    } else {
        Option::None
    };
    (title, summary, items, comp)
}

fn summary_for(kind: PlanKind, items: &[PlanItem]) -> String {
    use PlanKind::*;
    match kind {
        Titles => "8 个候选标题 · 风格各异".to_string(),
        Xhs => "标题 / 正文 / 标签 / 3 张配图 prompt".to_string(),
        Script => format!("口播稿 · {} 节拍", items.len()),
        Mindmap => format!("导图 · 1 根 + {} 分支", items.len()),
        Quotes => format!("{} 条金句", items.len()),
        Teardown => "钩子 + 结构 + 镜头 + 复用骨架 + 要点".to_string(),
        Video => "分镜 + 合成建议 + SRT + 视频制作包".to_string(),
        Image => "3 段图文 + 配图 prompt".to_string(),
        Ppt => "5-6 页演示大纲 + 视觉 prompt".to_string(),
        None => "原文二创".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_has_demo() {
        use PlanKind::*;
        for k in [None, Image, Video, Ppt, Teardown, Titles, Xhs, Script, Mindmap, Quotes] {
            let (title, summary, items, _comp) = load_demo_plan(k);
            assert!(!title.is_empty(), "demo {k:?} missing title");
            assert!(!summary.is_empty(), "demo {k:?} missing summary");
            assert!(!items.is_empty(), "demo {k:?} missing items");
        }
    }

    #[test]
    fn demo_video_has_4_scenes_with_required_fields() {
        let (title, items) = demo_video_scenes();
        assert_eq!(title, "夏日海边慢生活");
        assert_eq!(items.len(), 4);
        for it in &items {
            assert!(!it.get("id").is_empty());
            assert!(!it.get("duration_s").is_empty());
            assert!(!it.get("shot_type").is_empty());
            assert!(!it.get("description").is_empty());
            assert!(!it.get("voiceover").is_empty());
            assert!(!it.get("video_prompt").is_empty());
        }
    }

    #[test]
    fn demo_titles_returns_8_with_scores() {
        let (_title, items) = demo_titles_items();
        assert_eq!(items.len(), 8);
        for it in &items {
            let score: u32 = it.get("score").parse().unwrap_or(0);
            assert!(score > 0 && score <= 100, "score out of range: {score}");
        }
    }
}
