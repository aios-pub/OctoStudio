//! 10 scenario prompt templates + JSON schemas, ported 1:1 from the
//! splash `scenario_task()` / `scenario_input()` / `scenario_schema()`.
//!
//! Each scenario is built lazily (per [`scenario_prompts`] call) so the
//! `serde_json::json!` macro can use the `vec![]` array shorthand —
//! Rust forbids that in `const` context but allows it inside fn bodies.

use std::sync::OnceLock;

use serde_json::{json, Value};

use octostudio_core::PlanKind;

/// Per-scenario system + user prompt pair. The user prompt carries
/// the `intent` and optional `source` / `theme`; the system prompt
/// in [`task`] instructs the model to reply in strict JSON matching
/// [`schema`].
pub struct ScenarioPrompt {
    pub task: &'static str,
    pub input_template: fn(&ScenarioInput) -> Value,
    pub schema: OnceLock<Value>,
}

pub struct ScenarioInput {
    pub intent: String,
    pub source: String,
    pub theme: Option<String>,
    pub preset: Option<String>,
}

pub fn scenario_prompts(kind: PlanKind) -> &'static ScenarioPrompt {
    use PlanKind::*;
    match kind {
        None => &SCENARIO_NONE,
        Image => &SCENARIO_IMAGE,
        Video => &SCENARIO_VIDEO,
        Ppt => &SCENARIO_PPT,
        Teardown => &SCENARIO_TEARDOWN,
        Titles => &SCENARIO_TITLES,
        Xhs => &SCENARIO_XHS,
        Script => &SCENARIO_SCRIPT,
        Mindmap => &SCENARIO_MINDMAP,
        Quotes => &SCENARIO_QUOTES,
    }
}

// ---- 10 scenarios (each const holds the task + input template;
//      the schema is built lazily because json! arrays don't compile in const).

static SCENARIO_NONE: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名中文写作助手。根据用户意图,产出 1 个简短的标题。",
    input_template: |inp| json!({ "intent": inp.intent }),
    schema: OnceLock::new(),
};
static SCENARIO_IMAGE: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名图文内容策划。根据用户意图,产出 3-6 段图文大纲,每段含标题、正文、配图 prompt。",
    input_template: |inp| json!({ "intent": inp.intent, "theme": inp.theme }),
    schema: OnceLock::new(),
};
static SCENARIO_VIDEO: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名视频分镜编剧。根据用户意图,产出 3-6 镜分镜,每镜含时长、镜头类型、描述、旁白、视频 prompt。",
    input_template: |inp| json!({ "intent": inp.intent, "source": inp.source, "preset": inp.preset }),
    schema: OnceLock::new(),
};
static SCENARIO_PPT: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名演示文稿设计师。根据用户意图,产出 5-10 页演示大纲。",
    input_template: |inp| json!({ "intent": inp.intent, "theme": inp.theme }),
    schema: OnceLock::new(),
};
static SCENARIO_TEARDOWN: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名爆款视频分析师。拆解用户提供的视频文案/转录,产出结构化拆解。",
    input_template: |inp| json!({ "source": inp.source }),
    schema: OnceLock::new(),
};
static SCENARIO_TITLES: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名标题优化师。根据用户意图和参考,产出 5-8 个候选标题,各附风格标签和推荐指数 1-100。",
    input_template: |inp| json!({ "intent": inp.intent, "source": inp.source }),
    schema: OnceLock::new(),
};
static SCENARIO_XHS: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名小红书爆款笔记作者。根据用户意图,产出 1 篇带标题、正文、标签、配图 prompt 的笔记。",
    input_template: |inp| json!({ "intent": inp.intent, "theme": inp.theme }),
    schema: OnceLock::new(),
};
static SCENARIO_SCRIPT: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名口播稿作者。产出 1 段 15-180 秒的口播稿,含钩子、节拍、CTA。",
    input_template: |inp| json!({ "intent": inp.intent, "theme": inp.theme }),
    schema: OnceLock::new(),
};
static SCENARIO_MINDMAP: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名思维导图作者。根据用户意图/原文,产出 1 棵导图(1 根节点 + 3-6 个分支,每分支 2-5 个子节点)。",
    input_template: |inp| json!({ "intent": inp.intent, "source": inp.source }),
    schema: OnceLock::new(),
};
static SCENARIO_QUOTES: ScenarioPrompt = ScenarioPrompt {
    task: "你是一名金句作者。根据用户意图,产出 6-10 条金句,各附适用场景。",
    input_template: |inp| json!({ "intent": inp.intent }),
    schema: OnceLock::new(),
};

// ---- Schema builders (called lazily, return owned Value).

fn schema_none() -> Value {
    json!({
        "type": "object",
        "properties": { "title": { "type": "string", "maxLength": 40 } },
        "required": ["title"]
    })
}
fn schema_image() -> Value {
    json!({
        "type": "object",
        "properties": {
            "title":   { "type": "string", "maxLength": 40 },
            "summary": { "type": "string", "maxLength": 200 },
            "sections": { "type": "array", "minItems": 3, "maxItems": 6,
                "items": {
                    "type": "object",
                    "properties": {
                        "heading":      { "type": "string", "maxLength": 30 },
                        "body":         { "type": "string", "maxLength": 400 },
                        "image_prompt": { "type": "string", "maxLength": 300 }
                    },
                    "required": ["heading","body","image_prompt"]
                }
            }
        },
        "required": ["title","summary","sections"]
    })
}
fn schema_video() -> Value {
    json!({
        "type": "object",
        "properties": {
            "title":   { "type": "string", "maxLength": 40 },
            "logline": { "type": "string", "maxLength": 200 },
            "scenes": { "type": "array", "minItems": 3, "maxItems": 6,
                "items": {
                    "type": "object",
                    "properties": {
                        "id":           { "type": "string",  "maxLength": 10 },
                        "duration_s":   { "type": "integer", "minimum": 1, "maximum": 60 },
                        "shot_type":    { "type": "string",  "maxLength": 20 },
                        "description":  { "type": "string",  "maxLength": 200 },
                        "voiceover":    { "type": "string",  "maxLength": 300 },
                        "video_prompt": { "type": "string",  "maxLength": 300 }
                    },
                    "required": ["id","duration_s","shot_type","description","voiceover","video_prompt"]
                }
            }
        },
        "required": ["title","logline","scenes"]
    })
}
fn schema_ppt() -> Value {
    json!({
        "type": "object",
        "properties": {
            "deck_title": { "type": "string", "maxLength": 40 },
            "subtitle":   { "type": "string", "maxLength": 80 },
            "slides": { "type": "array", "minItems": 5, "maxItems": 10,
                "items": {
                    "type": "object",
                    "properties": {
                        "kind":    { "type": "string", "maxLength": 10 },
                        "title":   { "type": "string", "maxLength": 40 },
                        "bullets": { "type": "array", "minItems": 2, "maxItems": 5,
                            "items": { "type": "string", "maxLength": 60 } },
                        "notes":   { "type": "string", "maxLength": 160 },
                        "visual_prompt": { "type": "string", "maxLength": 240 }
                    },
                    "required": ["kind","title","bullets","notes","visual_prompt"]
                }
            }
        },
        "required": ["deck_title","subtitle","slides"]
    })
}
fn schema_teardown() -> Value {
    json!({
        "type": "object",
        "properties": {
            "video_title":  { "type": "string", "maxLength": 60 },
            "one_liner":    { "type": "string", "maxLength": 140 },
            "hook": {
                "type": "object",
                "properties": {
                    "pattern": { "type": "string", "maxLength": 40 },
                    "why":     { "type": "string", "maxLength": 200 }
                },
                "required": ["pattern","why"]
            },
            "structure": { "type": "array", "minItems": 3, "maxItems": 6,
                "items": {
                    "type": "object",
                    "properties": {
                        "time_hint": { "type": "string", "maxLength": 20 },
                        "purpose":   { "type": "string", "maxLength": 30 },
                        "summary":   { "type": "string", "maxLength": 160 }
                    },
                    "required": ["time_hint","purpose","summary"]
                }
            },
            "shot_language": { "type": "array", "minItems": 2, "maxItems": 5,
                "items": {
                    "type": "object",
                    "properties": {
                        "name":   { "type": "string", "maxLength": 20 },
                        "effect": { "type": "string", "maxLength": 120 }
                    },
                    "required": ["name","effect"]
                }
            },
            "golden_quotes": { "type": "array", "minItems": 1, "maxItems": 5,
                "items": { "type": "string", "maxLength": 80 } },
            "reusable": {
                "type": "object",
                "properties": {
                    "angle": { "type": "string", "maxLength": 140 },
                    "script_skeleton": { "type": "array", "minItems": 3, "maxItems": 6,
                        "items": { "type": "string", "maxLength": 90 } }
                },
                "required": ["angle","script_skeleton"]
            },
            "takeaways": { "type": "array", "minItems": 2, "maxItems": 4,
                "items": { "type": "string", "maxLength": 120 } }
        },
        "required": ["video_title","one_liner","hook","structure","shot_language",
                     "golden_quotes","reusable","takeaways"]
    })
}
fn schema_titles() -> Value {
    json!({
        "type": "object",
        "properties": {
            "titles": { "type": "array", "minItems": 5, "maxItems": 8,
                "items": {
                    "type": "object",
                    "properties": {
                        "text":      { "type": "string",  "maxLength": 40 },
                        "style_tag": { "type": "string",  "maxLength": 10 },
                        "score":     { "type": "integer", "minimum": 1, "maximum": 100 }
                    },
                    "required": ["text","style_tag","score"]
                }
            }
        },
        "required": ["titles"]
    })
}
fn schema_xhs() -> Value {
    json!({
        "type": "object",
        "properties": {
            "title":         { "type": "string", "maxLength": 24 },
            "body":          { "type": "string", "maxLength": 800 },
            "tags":          { "type": "array", "minItems": 3, "maxItems": 6,
                                "items": { "type": "string", "maxLength": 12 } },
            "image_prompts": { "type": "array", "minItems": 3, "maxItems": 3,
                                "items": { "type": "string", "maxLength": 200 } }
        },
        "required": ["title","body","tags","image_prompts"]
    })
}
fn schema_script() -> Value {
    json!({
        "type": "object",
        "properties": {
            "hook":  { "type": "string", "maxLength": 60 },
            "beats": { "type": "array", "minItems": 3, "maxItems": 6,
                "items": {
                    "type": "object",
                    "properties": {
                        "label": { "type": "string", "maxLength": 12 },
                        "line":  { "type": "string", "maxLength": 160 }
                    },
                    "required": ["label","line"]
                }
            },
            "cta":      { "type": "string",  "maxLength": 60 },
            "total_s":  { "type": "integer", "minimum": 15, "maximum": 180 },
            "tone":     { "type": "string",  "maxLength": 20 }
        },
        "required": ["hook","beats","cta","total_s","tone"]
    })
}
fn schema_mindmap() -> Value {
    json!({
        "type": "object",
        "properties": {
            "root": { "type": "string", "maxLength": 20 },
            "branches": { "type": "array", "minItems": 3, "maxItems": 6,
                "items": {
                    "type": "object",
                    "properties": {
                        "label":    { "type": "string", "maxLength": 20 },
                        "children": { "type": "array", "minItems": 2, "maxItems": 5,
                            "items": { "type": "string", "maxLength": 20 } }
                    },
                    "required": ["label","children"]
                }
            }
        },
        "required": ["root","branches"]
    })
}
fn schema_quotes() -> Value {
    json!({
        "type": "object",
        "properties": {
            "quotes": { "type": "array", "minItems": 6, "maxItems": 10,
                "items": {
                    "type": "object",
                    "properties": {
                        "text":      { "type": "string", "maxLength": 60 },
                        "use_case":  { "type": "string", "maxLength": 20 }
                    },
                    "required": ["text","use_case"]
                }
            }
        },
        "required": ["quotes"]
    })
}

/// Get the schema for a scenario, computing it once and caching.
pub fn schema_for(kind: PlanKind) -> &'static Value {
    let cell: &OnceLock<Value> = &scenario_prompts(kind).schema;
    cell.get_or_init(|| match kind {
        PlanKind::None     => schema_none(),
        PlanKind::Image    => schema_image(),
        PlanKind::Video    => schema_video(),
        PlanKind::Ppt      => schema_ppt(),
        PlanKind::Teardown => schema_teardown(),
        PlanKind::Titles   => schema_titles(),
        PlanKind::Xhs      => schema_xhs(),
        PlanKind::Script   => schema_script(),
        PlanKind::Mindmap  => schema_mindmap(),
        PlanKind::Quotes   => schema_quotes(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_has_a_prompt_and_schema() {
        use PlanKind::*;
        for k in [None, Image, Video, Ppt, Teardown, Titles, Xhs, Script, Mindmap, Quotes] {
            let p = scenario_prompts(k);
            assert!(!p.task.is_empty(), "task for {k:?} is empty");
            let s = schema_for(k);
            assert!(s.is_object(), "schema for {k:?} is not object");
        }
    }
}
