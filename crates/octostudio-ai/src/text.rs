//! TextClient — Agnes 3.0 Flash chat completions (OpenAI-compatible).
//!
//! Wraps [`crate::client::HttpClient`] and exposes:
//! - [`TextClient::chat`] — free-form chat with optional system prompt.
//! - [`TextClient::complete_json`] — strict JSON schema completion
//!   (uses `response_format: {type:"json_schema", ...}`).
//! - 7 panel helpers (1:1 from splash `ai_*` functions):
//!   [`TextClient::ai_gen_title`], [`ai_extract_keywords`], [`ai_summarize`],
//!   [`ai_style_variants`], [`ai_translate_zh_en`], [`ai_score_title`],
//!   [`ai_budget`].
//! - 10 scenario helper [`TextClient::ask_scenario`] that uses
//!   [`crate::prompts::scenario_prompts`] for the task + schema.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use octostudio_core::{OctostudioError, OctostudioResult, PlanKind};

use crate::client::HttpClient;
use crate::prompts::{scenario_prompts, ScenarioInput};
use crate::TEXT_MODEL;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "system" | "user" | "assistant"
    pub content: String,
}

impl ChatMessage {
    pub fn system(s: impl Into<String>) -> Self { ChatMessage { role: "system".into(), content: s.into() } }
    pub fn user(s: impl Into<String>) -> Self { ChatMessage { role: "user".into(), content: s.into() } }
    pub fn assistant(s: impl Into<String>) -> Self { ChatMessage { role: "assistant".into(), content: s.into() } }
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessageOut,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessageOut {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Clone)]
pub struct TextClient {
    pub http: HttpClient,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl TextClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        TextClient {
            http: HttpClient::new(api_key),
            model: TEXT_MODEL.to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }

    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.http.timeout = t;
        self
    }

    /// One-shot chat call.
    pub fn chat(&self, sys: &str, user: &str) -> OctostudioResult<String> {
        let req = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage::system(sys), ChatMessage::user(user)],
            max_tokens: Some(self.max_tokens),
            temperature: Some(self.temperature),
            response_format: None,
        };
        let resp: ChatResponse = self.http.post_json("/chat/completions", &req)?;
        let content = resp
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| OctostudioError::Ai("empty chat response".into()))?;
        Ok(content)
    }

    /// JSON-schema completion. `schema` is the standard JSON Schema
    /// object. Agnes uses `response_format: { type: "json_schema", ... }`
    /// (OpenAI-compatible). The returned value is the parsed object
    /// from the assistant message's `content` (a JSON string).
    pub fn complete_json(
        &self,
        sys: &str,
        user: &str,
        schema: &Value,
    ) -> OctostudioResult<Value> {
        let req = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage::system(sys), ChatMessage::user(user)],
            max_tokens: Some(self.max_tokens),
            temperature: Some(0.4), // more deterministic for JSON
            response_format: Some(json!({
                "type": "json_schema",
                "json_schema": { "name": "out", "schema": schema, "strict": true }
            })),
        };
        let resp: ChatResponse = self.http.post_json("/chat/completions", &req)?;
        let raw = resp
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| OctostudioError::Ai("empty JSON completion".into()))?;
        let val: Value = serde_json::from_str(&raw)
            .map_err(|e| OctostudioError::Ai(format!("parse JSON: {e}")))?;
        Ok(val)
    }

    // ---------- 7 panel helpers ----------

    pub fn ai_gen_title(&self, source: &str) -> OctostudioResult<Value> {
        let schema = json!({
            "type": "object",
            "properties": {
                "title": { "type": "string", "maxLength": 40 },
                "alternatives": { "type": "array", "minItems": 2, "maxItems": 4,
                    "items": { "type": "string", "maxLength": 40 } }
            },
            "required": ["title","alternatives"]
        });
        self.complete_json(
            "你是一名中文标题优化师。基于用户提供的内容,产出 1 个最佳标题 + 2-4 个备选标题。严格 JSON。",
            source,
            &schema,
        )
    }

    pub fn ai_extract_keywords(&self, source: &str) -> OctostudioResult<Value> {
        let schema = json!({
            "type": "object",
            "properties": {
                "keywords": { "type": "array", "minItems": 3, "maxItems": 8,
                    "items": { "type": "string", "maxLength": 12 } }
            },
            "required": ["keywords"]
        });
        self.complete_json("提取 3-8 个关键词(每词不超过 12 字)。", source, &schema)
    }

    pub fn ai_summarize(&self, source: &str) -> OctostudioResult<Value> {
        let schema = json!({
            "type": "object",
            "properties": {
                "summary":    { "type": "string", "maxLength": 200 },
                "key_points": { "type": "array", "minItems": 2, "maxItems": 5,
                    "items": { "type": "string", "maxLength": 60 } }
            },
            "required": ["summary","key_points"]
        });
        self.complete_json(
            "为内容生成 1 句摘要(≤200 字)+ 2-5 条要点(每条 ≤60 字)。",
            source,
            &schema,
        )
    }

    pub fn ai_style_variants(&self, source: &str) -> OctostudioResult<Value> {
        let schema = json!({
            "type": "object",
            "properties": {
                "variants": { "type": "array", "minItems": 4, "maxItems": 4,
                    "items": {
                        "type": "object",
                        "properties": {
                            "style": { "type": "string", "maxLength": 12 },
                            "body":  { "type": "string", "maxLength": 400 }
                        },
                        "required": ["style","body"]
                    }
                }
            },
            "required": ["variants"]
        });
        self.complete_json(
            "把内容改写成 4 种风格(理性 / 抒情 / 犀利 / 温暖),每种 ≤400 字。",
            source,
            &schema,
        )
    }

    pub fn ai_translate_zh_en(&self, source: &str) -> OctostudioResult<Value> {
        let schema = json!({
            "type": "object",
            "properties": {
                "zh": { "type": "string", "maxLength": 600 },
                "en": { "type": "string", "maxLength": 600 }
            },
            "required": ["zh","en"]
        });
        self.complete_json(
            "为内容产出中英对照(各 ≤600 字),保留原意和语气。",
            source,
            &schema,
        )
    }

    pub fn ai_score_title(&self, title: &str) -> OctostudioResult<Value> {
        let schema = json!({
            "type": "object",
            "properties": {
                "score":       { "type": "integer", "minimum": 0, "maximum": 100 },
                "suggestions": { "type": "array", "minItems": 2, "maxItems": 4,
                    "items": { "type": "string", "maxLength": 60 } }
            },
            "required": ["score","suggestions"]
        });
        self.complete_json(
            "为标题打分 0-100,2-4 条改进建议。",
            title,
            &schema,
        )
    }

    /// Local "model budget" — Agnes doesn't expose a per-day token quota
    /// API, so this is a friendly roll-up of [`crate::usage`]. Kept here
    /// for symmetry with splash's `model.budget` call.
    pub fn ai_budget(&self) -> &'static str {
        "见 Settings 屏配额卡片(usage.json)"
    }

    // ---------- 10 scenario helper ----------

    pub fn ask_scenario(
        &self,
        kind: PlanKind,
        intent: &str,
        source: &str,
        theme: Option<&str>,
        preset: Option<&str>,
    ) -> OctostudioResult<Value> {
        let p = scenario_prompts(kind);
        let input = (p.input_template)(&ScenarioInput {
            intent: intent.to_string(),
            source: source.to_string(),
            theme: theme.map(str::to_string),
            preset: preset.map(str::to_string),
        });
        let user_prompt = serde_json::to_string(&input).map_err(OctostudioError::Json)?;
        let schema = crate::prompts::schema_for(kind);
        self.complete_json(p.task, &user_prompt, schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_message_builders() {
        let m = ChatMessage::user("hi");
        assert_eq!(m.role, "user");
        assert_eq!(m.content, "hi");
    }
}
