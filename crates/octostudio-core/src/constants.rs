//! Static lookup tables ported 1:1 from the splash bundle (v0.5-alpha-ui-polish).
//!
//! - 10 [`Scenario`] entries (one per `PlanKind`, including the default
//!   "原文二创" which uses `id: ""`).
//! - 11 [`Theme`] entries (article-style tones, id `""` = "不限").
//! - 6 [`Preset`] entries (video styles, id `""` = "不限").
//! - 8 [`Style`] entries (article style chips, no id — just label strings).

use crate::enums::PlanKind;

/// One scenario card. `id` matches `PlanKind::as_id()` exactly.
#[derive(Debug, Clone, Copy)]
pub struct Scenario {
    pub id: &'static str,
    pub name: &'static str,
    pub emoji: &'static str,
    pub hint: &'static str,
}

pub const SCENARIOS: &[Scenario] = &[
    Scenario { id: "",         name: "原文二创", emoji: "📝", hint: "粘贴原文,改写/翻译/总结/评论/润色" },
    Scenario { id: "image",     name: "图文文章", emoji: "📷", hint: "一句话生成图文大纲与配图 prompt" },
    Scenario { id: "video",     name: "视频分镜", emoji: "🎬", hint: "一句话生成 4-6 镜分镜与合成包" },
    Scenario { id: "ppt",       name: "PPT 演示", emoji: "📊", hint: "一句话生成演示大纲,导出 Marp" },
    Scenario { id: "teardown",  name: "拆解视频", emoji: "🔍", hint: "拆解爆款视频,提取可复用骨架" },
    Scenario { id: "titles",    name: "标题工坊", emoji: "🏷️", hint: "8 个候选标题,附推荐指数" },
    Scenario { id: "xhs",       name: "小红书笔记", emoji: "📕", hint: "标题/正文/标签/配图一步到位" },
    Scenario { id: "script",    name: "口播稿", emoji: "🎤", hint: "钩子/节拍/CTA,真人出镜即用" },
    Scenario { id: "mindmap",   name: "思维导图", emoji: "🧠", hint: "主题或原文变导图,导 markmap" },
    Scenario { id: "quotes",    name: "金句语录", emoji: "✨", hint: "8 条金句,注明适用场景" },
];

/// Article style theme (id `""` = "不限").
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub id: &'static str,
    pub name: &'static str,
    pub emoji: &'static str,
    pub tone: &'static str,
}

pub const THEMES: &[Theme] = &[
    Theme { id: "",       name: "不限",         emoji: "🎨", tone: "自由发挥,清晰易读。" },
    Theme { id: "gzh",     name: "公众号深度",   emoji: "📰", tone: "深度长文口吻,层层递进,克制理性。" },
    Theme { id: "list",    name: "干货清单",     emoji: "✅", tone: "清单体,每段一个要点,动词开头。" },
    Theme { id: "prose",   name: "情感散文",     emoji: "🌙", tone: "细腻抒情,具象细节,节奏舒缓。" },
    Theme { id: "xhs",     name: "小红书种草",   emoji: "🍑", tone: "口语化短句,适度 emoji,真诚分享感。" },
    Theme { id: "zhihu",   name: "知乎科普",     emoji: "🔬", tone: "先结论后论证,术语准确,科普口吻。" },
    Theme { id: "tech",    name: "科技评测",     emoji: "🤖", tone: "参数与体验并重,客观中立的评测口吻。" },
    Theme { id: "travel",  name: "旅行游记",     emoji: "🧭", tone: "移步换景,人文观察,游记口吻。" },
    Theme { id: "food",    name: "美食探店",     emoji: "🍜", tone: "感官描写为主,实用信息收尾。" },
    Theme { id: "career",  name: "职场干货",     emoji: "💼", tone: "场景化案例,方法论清晰,职场口吻。" },
    Theme { id: "poem",    name: "诗歌意象",     emoji: "🕯️", tone: "诗化语言,意象密度高,分行呼吸感。" },
];

/// Video style preset (id `""` = "不限").
#[derive(Debug, Clone, Copy)]
pub struct Preset {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
}

pub const PRESETS: &[Preset] = &[
    Preset { id: "",        name: "不限",     desc: "" },
    Preset { id: "cinema",  name: "电影感",   desc: "电影级调色、稳定运镜、浅景深" },
    Preset { id: "vlog",    name: "Vlog",     desc: "手持跟拍、自然光、生活化视角" },
    Preset { id: "guofeng", name: "国风",     desc: "留白构图、水墨色调、古典意象" },
    Preset { id: "cyber",   name: "赛博",     desc: "霓虹光效、夜景、高对比冷暖" },
    Preset { id: "heal",    name: "治愈",     desc: "柔光、暖调、慢节奏空镜" },
];

/// Article style chip. Plain strings — no `id` because the splash storage
/// is just the label.
pub type Style = &'static str;
pub const STYLES: &[Style] = &[
    "改写", "翻译", "总结", "评论", "润色", "续写", "扩写", "小红书体",
];

// ---------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------

/// Resolve a scenario by its [`PlanKind`].
pub fn scenario_of(kind: PlanKind) -> &'static Scenario {
    SCENARIOS
        .iter()
        .find(|s| s.id == kind.as_id())
        .expect("SCENARIOS must contain an entry for every PlanKind")
}

/// Resolve a theme by id, returning `None` for the empty / "不限" id.
pub fn theme_of(id: &str) -> Option<&'static Theme> {
    if id.is_empty() { return None; }
    THEMES.iter().find(|t| t.id == id)
}

/// Resolve a video preset by id, returning `None` for the empty / "不限" id.
pub fn preset_of(id: &str) -> Option<&'static Preset> {
    if id.is_empty() { return None; }
    PRESETS.iter().find(|p| p.id == id)
}
