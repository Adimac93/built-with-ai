use std::time::Duration;

use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::error::AgentError;

const MODEL: &str = "gemini-flash-latest";
const METADATA_TOKEN_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
/// One retry on transient failures; ADK relied on the genai SDK's internal
/// retries, this keeps rough parity without a backoff dependency.
const MAX_ATTEMPTS: u32 = 2;

/// Output mode for a Gemini call.
pub enum Output<'a> {
    /// Free-form text.
    Text,
    /// `responseMimeType: application/json`, optionally schema-constrained
    /// (schema omitted where the shape has open maps the OpenAPI subset
    /// cannot express — the evaluator's `scores`).
    Json(Option<&'a Value>),
}

#[derive(Clone)]
pub enum GeminiAuth {
    /// Google AI Studio key — local development.
    ApiKey(String),
    /// Vertex AI: `GOOGLE_CLOUD_ACCESS_TOKEN` env override, else the GCP
    /// metadata server (Cloud Run service account, `aiplatform.user`).
    Vertex,
}

#[derive(Clone)]
pub struct GeminiClient {
    client: reqwest::Client,
    /// URL prefix up to (excluding) `/models/{model}:generateContent`.
    base_url: String,
    auth: GeminiAuth,
}

impl GeminiClient {
    /// Builds the client from the environment. `GEMINI_BASE_URL` overrides the
    /// endpoint (used by integration tests to point at a mock server).
    pub fn from_env() -> Self {
        let api_key = std::env::var("GEMINI_API_KEY").ok().filter(|k| !k.is_empty());
        let auth = match &api_key {
            Some(key) => GeminiAuth::ApiKey(key.clone()),
            None => GeminiAuth::Vertex,
        };
        let base_url = std::env::var("GEMINI_BASE_URL").ok().unwrap_or_else(|| {
            match &auth {
                GeminiAuth::ApiKey(_) => {
                    "https://generativelanguage.googleapis.com/v1beta".to_string()
                }
                GeminiAuth::Vertex => {
                    let project = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap_or_default();
                    // GOOGLE_CLOUD_LOCATION is `global` → the global Vertex host.
                    format!(
                        "https://aiplatform.googleapis.com/v1/projects/{project}/locations/global/publishers/google"
                    )
                }
            }
        });
        Self::new(base_url, auth)
    }

    pub fn new(base_url: String, auth: GeminiAuth) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("reqwest client"),
            base_url,
            auth,
        }
    }

    /// One Gemini call returning the concatenated text of the first candidate.
    /// `system` maps to `systemInstruction` (ADK put agent instructions there
    /// for the normalizer; other steps inline everything into the user turn).
    /// JSON output modes are the equivalent of ADK's `output_schema`.
    pub async fn generate_text(
        &self,
        system: Option<&str>,
        user: &str,
        output: Output<'_>,
    ) -> Result<String, AgentError> {
        let mut body = json!({
            "contents": [{ "role": "user", "parts": [{ "text": user }] }],
        });
        if let Some(system) = system {
            body["systemInstruction"] = json!({ "parts": [{ "text": system }] });
        }
        match output {
            Output::Text => {}
            Output::Json(schema) => {
                let mut config = json!({ "responseMimeType": "application/json" });
                if let Some(schema) = schema {
                    config["responseSchema"] = schema.clone();
                }
                body["generationConfig"] = config;
            }
        }

        let url = format!("{}/models/{MODEL}:generateContent", self.base_url);
        let mut last_error = String::new();

        for attempt in 1..=MAX_ATTEMPTS {
            match self.attempt(&url, &body).await {
                Ok(text) => return Ok(text),
                Err((retryable, detail)) => {
                    tracing::warn!("Gemini call attempt {attempt} failed: {detail}");
                    last_error = detail;
                    if !retryable {
                        break;
                    }
                }
            }
        }
        Err(AgentError::internal(format!("Gemini call failed: {last_error}")))
    }

    /// Structured call: JSON output parsed into `T`.
    pub async fn generate_json<T: DeserializeOwned>(
        &self,
        user: &str,
        response_schema: Option<&Value>,
    ) -> Result<T, AgentError> {
        let text = self
            .generate_text(None, user, Output::Json(response_schema))
            .await?;
        serde_json::from_str(&text).map_err(|err| {
            tracing::error!("Gemini returned unparseable JSON: {err}; text: {text}");
            AgentError::internal(format!("Gemini returned invalid JSON: {err}"))
        })
    }

    /// Single request attempt. Error tuple: (retryable, detail).
    async fn attempt(&self, url: &str, body: &Value) -> Result<String, (bool, String)> {
        let mut request = self.client.post(url).json(body);
        request = match &self.auth {
            GeminiAuth::ApiKey(key) => request.header("x-goog-api-key", key),
            GeminiAuth::Vertex => {
                let token = self.vertex_token().await.map_err(|e| (true, e))?;
                request.bearer_auth(token)
            }
        };

        let response = request.send().await.map_err(|err| (true, err.to_string()))?;
        let status = response.status();
        let data: Value = response.json().await.map_err(|err| (true, err.to_string()))?;

        if !status.is_success() {
            let detail = data
                .pointer("/error/message")
                .and_then(Value::as_str)
                .unwrap_or_else(|| status.as_str())
                .to_string();
            let retryable = status.is_server_error() || status.as_u16() == 429;
            return Err((retryable, detail));
        }

        let text: String = data
            .pointer("/candidates/0/content/parts")
            .and_then(Value::as_array)
            .map(|parts| {
                parts
                    .iter()
                    .filter_map(|part| part.get("text").and_then(Value::as_str))
                    .collect()
            })
            .unwrap_or_default();

        if text.is_empty() {
            return Err((true, format!("empty model response: {data}")));
        }
        Ok(text)
    }

    async fn vertex_token(&self) -> Result<String, String> {
        if let Ok(token) = std::env::var("GOOGLE_CLOUD_ACCESS_TOKEN") {
            if !token.is_empty() {
                return Ok(token);
            }
        }
        let response = self
            .client
            .get(METADATA_TOKEN_URL)
            .header("Metadata-Flavor", "Google")
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|err| format!("metadata server unreachable: {err}"))?;
        if !response.status().is_success() {
            return Err(format!("metadata server returned {}", response.status()));
        }
        let data: Value = response
            .json()
            .await
            .map_err(|err| format!("metadata token parse failed: {err}"))?;
        data.get("access_token")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| "metadata server returned no access token".to_string())
    }
}
