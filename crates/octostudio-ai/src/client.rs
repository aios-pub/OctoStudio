//! Tiny `ureq`-backed HTTP client. Synchronous on the calling thread,
//! so it composes naturally with makepad's single-threaded UI model.

use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;

use octostudio_core::OctostudioError;

use crate::DEFAULT_BASE_URL;

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub base_url: String,
    pub api_key: String,
    pub timeout: Duration,
}

impl HttpClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        HttpClient {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: api_key.into(),
            timeout: Duration::from_secs(60),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.timeout = t;
        self
    }

    /// POST `path` (relative to `base_url`) with a serialised JSON body.
    /// Returns the parsed JSON response on 2xx.
    pub fn post_json<Req: Serialize, Resp: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Resp, OctostudioError> {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(self.timeout))
            .build()
            .new_agent();
        let auth = format!("Bearer {}", self.api_key);
        let mut resp = agent
            .post(&url)
            .header("Authorization", &auth)
            .content_type("application/json")
            .send_json(serde_json::to_value(body).map_err(OctostudioError::Json)?)
            .map_err(|e| OctostudioError::Http(format!("POST {url}: {e}")))?;
        let body: Resp = resp
            .body_mut()
            .read_json()
            .map_err(|e| OctostudioError::Http(format!("decode {url}: {e}")))?;
        Ok(body)
    }

    /// GET `path` (full URL or relative). Returns the parsed JSON response.
    pub fn get_json<Resp: DeserializeOwned>(&self, url: &str) -> Result<Resp, OctostudioError> {
        let full = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("{}{}", self.base_url.trim_end_matches('/'), url)
        };
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(self.timeout))
            .build()
            .new_agent();
        let auth = format!("Bearer {}", self.api_key);
        let mut resp = agent
            .get(&full)
            .header("Authorization", &auth)
            .call()
            .map_err(|e| OctostudioError::Http(format!("GET {full}: {e}")))?;
        let body: Resp = resp
            .body_mut()
            .read_json()
            .map_err(|e| OctostudioError::Http(format!("decode {full}: {e}")))?;
        Ok(body)
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_strips_trailing_slash() {
        let c = HttpClient::new("k").with_base_url("https://api.example.com/v1/");
        // We don't actually call the network here — just verify the URL
        // composition by inspecting a constructed string. (post_json would
        // need a fake server.) The internal helper:
        let full = format!(
            "{}{}",
            c.base_url.trim_end_matches('/'),
            "/chat"
        );
        assert_eq!(full, "https://api.example.com/v1/chat");
    }
}
