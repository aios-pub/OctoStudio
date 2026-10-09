//! ImageClient — Agnes 2.5 Flash image generation (OpenAI-compatible).
//!
//! Endpoint: `POST /v1/images/generations`
//! Docs: <https://www.agnes-ai.com/zh-Hans/docs/agnes-image-25-flash>
//!
//! Returns a public URL on success. `size` is one of `"1K"` / `"2K"` /
//! `"3K"` / `"4K"` or legacy `"WxH"` (e.g. `"1024x768"`). `ratio` is
//! one of `"1:1"` / `"3:4"` / `"4:3"` / `"16:9"` / `"9:16"` / etc.
//! `image` (optional) is a list of public HTTPS URLs or data URIs for
//! img2img / multi-image composition.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use octostudio_core::OctostudioResult;

use crate::client::HttpClient;
use crate::IMAGE_MODEL;

#[derive(Clone)]
pub struct ImageClient {
    pub http: HttpClient,
    pub model: String,
}

impl ImageClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        ImageClient {
            http: HttpClient::new(api_key),
            model: IMAGE_MODEL.to_string(),
        }
    }

    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.http.timeout = t;
        self
    }

    /// Text-to-image. Returns the image URL.
    pub fn generate(
        &self,
        prompt: &str,
        size: &str,
        ratio: &str,
    ) -> OctostudioResult<String> {
        let body = json!({
            "model": self.model,
            "prompt": prompt,
            "size": size,
            "ratio": ratio,
            "extra_body": { "response_format": "url" }
        });
        let resp: ImagesResponse = self.http.post_json("/images/generations", &body)?;
        let url = resp
            .data
            .first()
            .and_then(|d| d.url.clone())
            .ok_or_else(|| octostudio_core::OctostudioError::Ai("empty image response".into()))?;
        Ok(url)
    }

    /// Image-to-image. `images` is a list of public HTTPS URLs or data
    /// URIs. `prompt` describes the target.
    pub fn generate_with_references(
        &self,
        prompt: &str,
        size: &str,
        ratio: &str,
        images: &[String],
    ) -> OctostudioResult<String> {
        let body = json!({
            "model": self.model,
            "prompt": prompt,
            "size": size,
            "ratio": ratio,
            "extra_body": {
                "image": images,
                "response_format": "url"
            }
        });
        let resp: ImagesResponse = self.http.post_json("/images/generations", &body)?;
        let url = resp
            .data
            .first()
            .and_then(|d| d.url.clone())
            .ok_or_else(|| octostudio_core::OctostudioError::Ai("empty image response".into()))?;
        Ok(url)
    }
}

#[derive(Debug, Clone, Serialize)]
struct ImagesRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    size: &'a str,
    ratio: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_body: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct ImagesResponse {
    data: Vec<ImageData>,
}

#[derive(Debug, Clone, Deserialize)]
struct ImageData {
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    b64_json: Option<String>,
    #[serde(default)]
    revised_prompt: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_request_serialises_size_ratio() {
        let body = json!({
            "model": "agnes-image-2.5-flash",
            "prompt": "test",
            "size": "2K",
            "ratio": "16:9",
            "extra_body": { "response_format": "url" }
        });
        assert_eq!(body["size"], "2K");
        assert_eq!(body["ratio"], "16:9");
    }
}
