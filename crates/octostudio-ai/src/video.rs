//! VideoClient — Agnes 2.5 video generation (OpenAI Videos-compatible).
//!
//! Endpoints (OpenAI Videos + Agnes-specific poll):
//! - Create: `POST /v1/videos`
//! - Poll:   `GET  /agnesapi?video_id=<ID>&model_name=agnes-video-2.5`
//!
//! Agnes is **async only**: create returns a `video_id` immediately;
//! poll must be called until `status` is `completed` (with `url` in the
//! response body) or `failed` (with an error message).
//!
//! Modes: `text` / `keyframe` (first/last_frame) / `reference` (multi-media).
//! `size`: `720P` / `1080P` / `1K` / `2K`. `seconds`: `"4"`–`"12"`.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use octostudio_core::OctostudioError;
use octostudio_core::OctostudioResult;

use crate::client::HttpClient;
use crate::VIDEO_MODEL;

#[derive(Debug, Clone)]
pub enum VideoStatus {
    Queued,
    Running(u8), // 0..=100
    Completed(String), // url
    Failed(String), // error message
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoCreateRequest<'a> {
    pub model: &'a str,
    pub prompt: &'a str,
    pub mode: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_frame: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_frame: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoCreateResponse {
    pub id: Option<String>,
    pub task_id: Option<String>,
    pub video_id: Option<String>,
    pub status: Option<String>,
}

impl VideoCreateResponse {
    /// The canonical id to poll. Agnes returns multiple id fields
    /// depending on mode — prefer `video_id` > `task_id` > `id`.
    pub fn canonical_id(&self) -> Option<String> {
        self.video_id
            .clone()
            .or_else(|| self.task_id.clone())
            .or_else(|| self.id.clone())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoPollResponse {
    pub status: Option<String>,
    pub progress: Option<u8>,
    pub url: Option<String>,
    pub error: Option<String>,
    pub completed_at: Option<String>,
}

impl VideoPollResponse {
    pub fn into_status(self) -> VideoStatus {
        let s = self.status.as_deref().unwrap_or("");
        match s {
            "completed" => VideoStatus::Completed(self.url.unwrap_or_default()),
            "failed"    => VideoStatus::Failed(self.error.unwrap_or_else(|| "unknown".into())),
            "running" | "processing" => VideoStatus::Running(self.progress.unwrap_or(0)),
            _           => VideoStatus::Queued,
        }
    }
}

#[derive(Clone)]
pub struct VideoClient {
    pub http: HttpClient,
    pub model: String,
}

impl VideoClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        VideoClient {
            http: HttpClient::new(api_key),
            model: VIDEO_MODEL.to_string(),
        }
    }

    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.http.timeout = t;
        self
    }

    /// Submit a text-to-video job. Returns the `video_id` to poll.
    pub fn create_text(
        &self,
        prompt: &str,
        seconds: &str,
        size: &str,
        aspect_ratio: &str,
    ) -> OctostudioResult<String> {
        let req = VideoCreateRequest {
            model: &self.model,
            prompt,
            mode: "text",
            seconds: Some(seconds),
            size: Some(size),
            aspect_ratio: Some(aspect_ratio),
            first_frame: None,
            last_frame: None,
            seed: None,
            n: Some(1),
        };
        let resp: VideoCreateResponse = self.http.post_json("/videos", &req)?;
        resp.canonical_id()
            .ok_or_else(|| OctostudioError::Ai("no video id in create response".into()))
    }

    /// Submit a keyframe job (text-to-video with first/last frame).
    pub fn create_keyframe(
        &self,
        prompt: &str,
        first_frame: &str,
        last_frame: Option<&str>,
        seconds: &str,
        size: &str,
        aspect_ratio: &str,
    ) -> OctostudioResult<String> {
        let req = VideoCreateRequest {
            model: &self.model,
            prompt,
            mode: "keyframe",
            seconds: Some(seconds),
            size: Some(size),
            aspect_ratio: Some(aspect_ratio),
            first_frame: Some(first_frame),
            last_frame,
            seed: None,
            n: Some(1),
        };
        let resp: VideoCreateResponse = self.http.post_json("/videos", &req)?;
        resp.canonical_id()
            .ok_or_else(|| OctostudioError::Ai("no video id in create response".into()))
    }

    /// Poll a job. Returns the current [`VideoStatus`].
    pub fn poll(&self, video_id: &str) -> OctostudioResult<VideoStatus> {
        let path = format!(
            "/agnesapi?video_id={video_id}&model_name={model}",
            model = self.model
        );
        let resp: VideoPollResponse = self.http.get_json(&path)?;
        Ok(resp.into_status())
    }

    /// Create a text-to-video job and poll until done (or `max_wait`
    /// elapses, polling every `interval`). Returns the final status.
    ///
    /// Used by the demo button on Home — the plan-view video flow will
    /// run this on a background timer (C7 leaves the timer logic for the
    /// plan-view integration in a follow-up).
    pub fn create_and_wait(
        &self,
        prompt: &str,
        seconds: &str,
        size: &str,
        aspect_ratio: &str,
        interval: Duration,
        max_wait: Duration,
    ) -> OctostudioResult<VideoStatus> {
        let id = self.create_text(prompt, seconds, size, aspect_ratio)?;
        let start = std::time::Instant::now();
        loop {
            std::thread::sleep(interval);
            match self.poll(&id)? {
                VideoStatus::Completed(_) | VideoStatus::Failed(_) => return Ok(self.poll(&id)?),
                _ if start.elapsed() > max_wait => {
                    return Err(OctostudioError::Ai(format!(
                        "video {id} timed out after {}s",
                        max_wait.as_secs()
                    )))
                }
                _ => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_id_prefers_video_id() {
        let r = VideoCreateResponse {
            id: Some("task_1".into()),
            task_id: Some("task_2".into()),
            video_id: Some("vid_3".into()),
            status: Some("queued".into()),
        };
        assert_eq!(r.canonical_id().as_deref(), Some("vid_3"));
    }

    #[test]
    fn canonical_id_falls_back_to_task_id_then_id() {
        let r = VideoCreateResponse {
            id: Some("task_1".into()),
            task_id: Some("task_2".into()),
            video_id: None,
            status: Some("queued".into()),
        };
        assert_eq!(r.canonical_id().as_deref(), Some("task_2"));

        let r = VideoCreateResponse {
            id: Some("only_id".into()),
            task_id: None,
            video_id: None,
            status: Some("queued".into()),
        };
        assert_eq!(r.canonical_id().as_deref(), Some("only_id"));
    }

    #[test]
    fn poll_response_into_status() {
        let r = VideoPollResponse {
            status: Some("completed".into()),
            progress: Some(100),
            url: Some("https://x/y.mp4".into()),
            error: None,
            completed_at: None,
        };
        match r.into_status() {
            VideoStatus::Completed(u) => assert_eq!(u, "https://x/y.mp4"),
            _ => panic!("expected Completed"),
        }
    }
}
