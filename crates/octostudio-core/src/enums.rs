//! Core enums for screens, plan kinds, export formats, sort modes.
//!
//! Each enum carries both a programmatic `as_*` and a `from_*` (string id) that
//! matches the splash serialization format, so `works.json` is byte-compatible
//! across the migration.

use serde::{Deserialize, Serialize};

/// Top-level screen (tab). Six entries in v0.6 (Settings replaces "Export"
/// as a tab; Export is reached via a Plan/Studio top-right button).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Screen {
    Home,
    Compose,
    Studio,
    Plan,
    Export,
    Settings,
}

impl Screen {
    pub const ALL: [Screen; 6] = [
        Screen::Home,
        Screen::Compose,
        Screen::Studio,
        Screen::Plan,
        Screen::Export,
        Screen::Settings,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Screen::Home => "广场",
            Screen::Compose => "开始一段创作",
            Screen::Studio => "工坊",
            Screen::Plan => "计划",
            Screen::Export => "导出",
            Screen::Settings => "设置",
        }
    }

    pub fn tab_label(self) -> &'static str {
        match self {
            Screen::Home => "广场",
            Screen::Compose => "录入",
            Screen::Studio => "工坊",
            Screen::Plan => "计划",
            Screen::Export => "导出",
            Screen::Settings => "设置",
        }
    }

    pub fn tab_emoji(self) -> &'static str {
        match self {
            Screen::Home => "◐",
            Screen::Compose => "✎",
            Screen::Studio => "◇",
            Screen::Plan => "▤",
            Screen::Export => "⬆",
            Screen::Settings => "⚙",
        }
    }
}

/// Scenario identifier. `None` represents the default "原文二创" mode where
/// the user pastes a long source text and picks a style chip (改写/翻译/…).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanKind {
    #[serde(rename = "")]
    None,
    Image,
    Video,
    Ppt,
    Teardown,
    Titles,
    Xhs,
    Script,
    Mindmap,
    Quotes,
}

impl PlanKind {
    pub fn as_id(self) -> &'static str {
        match self {
            PlanKind::None => "",
            PlanKind::Image => "image",
            PlanKind::Video => "video",
            PlanKind::Ppt => "ppt",
            PlanKind::Teardown => "teardown",
            PlanKind::Titles => "titles",
            PlanKind::Xhs => "xhs",
            PlanKind::Script => "script",
            PlanKind::Mindmap => "mindmap",
            PlanKind::Quotes => "quotes",
        }
    }

    pub fn from_id(s: &str) -> Self {
        match s {
            "" => PlanKind::None,
            "image" => PlanKind::Image,
            "video" => PlanKind::Video,
            "ppt" => PlanKind::Ppt,
            "teardown" => PlanKind::Teardown,
            "titles" => PlanKind::Titles,
            "xhs" => PlanKind::Xhs,
            "script" => PlanKind::Script,
            "mindmap" => PlanKind::Mindmap,
            "quotes" => PlanKind::Quotes,
            _ => PlanKind::None,
        }
    }

    pub fn name(self) -> &'static str {
        use crate::constants::SCENARIOS;
        SCENARIOS
            .iter()
            .find(|s| s.id == self.as_id())
            .map(|s| s.name)
            .unwrap_or("原文二创")
    }

    pub fn emoji(self) -> &'static str {
        use crate::constants::SCENARIOS;
        SCENARIOS
            .iter()
            .find(|s| s.id == self.as_id())
            .map(|s| s.emoji)
            .unwrap_or("📝")
    }

    pub fn short(self) -> &'static str {
        match self {
            PlanKind::None => "文章",
            PlanKind::Image => "图文",
            PlanKind::Video => "视频",
            PlanKind::Ppt => "演示",
            PlanKind::Teardown => "拆解",
            PlanKind::Titles => "标题",
            PlanKind::Xhs => "小红书",
            PlanKind::Script => "口播",
            PlanKind::Mindmap => "导图",
            PlanKind::Quotes => "金句",
        }
    }
}

/// 8 export formats the splash supports; one (or several) per plan kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    Markdown,
    Wechat,
    Notion,
    Srt,
    Pack,
    Marp,
    Markmap,
}

impl ExportFormat {
    pub fn as_id(self) -> &'static str {
        match self {
            ExportFormat::Markdown => "markdown",
            ExportFormat::Wechat => "wechat",
            ExportFormat::Notion => "notion",
            ExportFormat::Srt => "srt",
            ExportFormat::Pack => "pack",
            ExportFormat::Marp => "marp",
            ExportFormat::Markmap => "markmap",
        }
    }

    pub fn from_id(s: &str) -> Self {
        match s {
            "markdown" => ExportFormat::Markdown,
            "wechat" => ExportFormat::Wechat,
            "notion" => ExportFormat::Notion,
            "srt" => ExportFormat::Srt,
            "pack" => ExportFormat::Pack,
            "marp" => ExportFormat::Marp,
            "markmap" => ExportFormat::Markmap,
            _ => ExportFormat::Markdown,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ExportFormat::Markdown => "Markdown",
            ExportFormat::Wechat => "公众号",
            ExportFormat::Notion => "Notion",
            ExportFormat::Srt => "SRT 字幕",
            ExportFormat::Pack => "视频制作包",
            ExportFormat::Marp => "Marp 演示",
            ExportFormat::Markmap => "Markmap 导图",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            ExportFormat::Srt => "全选复制,保存为 subtitle.srt(UTF-8),放视频同目录。",
            ExportFormat::Pack => {
                "视频制作包:逐镜 prompt + SRT + 合成建议 + ffmpeg 命令,照做即可成片。"
            }
            ExportFormat::Marp => "全选复制,粘到 marp.app 或 VSCode Marp 插件即可放映/导出 PDF。",
            ExportFormat::Markmap => "全选复制,粘到 markmap.js.org/repl 生成导图。",
            ExportFormat::Wechat => "纯文本无标记,直接粘进公众号/小红书编辑器。",
            ExportFormat::Notion => "Markdown 格式,粘进 Notion 自动转换。",
            ExportFormat::Markdown => "全选下面文本 → 复制,粘到目标平台。",
        }
    }
}

/// 3 sort orders for the home works list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SortMode {
    UpdatedDesc,
    UpdatedAsc,
    ByKind,
}

impl SortMode {
    pub fn as_id(self) -> &'static str {
        match self {
            SortMode::UpdatedDesc => "updated_desc",
            SortMode::UpdatedAsc => "updated_asc",
            SortMode::ByKind => "kind",
        }
    }

    pub fn from_id(s: &str) -> Self {
        match s {
            "updated_asc" => SortMode::UpdatedAsc,
            "kind" => SortMode::ByKind,
            _ => SortMode::UpdatedDesc,
        }
    }
}
