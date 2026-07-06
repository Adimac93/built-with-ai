//! Server side of the fullstack app: the axum server and the backend logic
//! ported from the former `apps/api` crate — agent proxy and Google
//! Speech-to-Text.
#![cfg(feature = "server")]

use std::time::Duration;

use serde_json::{json, Value};

const METADATA_TOKEN_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
const SPEECH_API_URL: &str = "https://speech.googleapis.com/v1/speech:recognize";
/// Matches the old gateway's axios-era timeout for the agent call.
const AGENT_TIMEOUT: Duration = Duration::from_secs(120);
/// Problem statements shorter than this are rejected (old `@MinLength(10)`).
const MIN_PROBLEM_LENGTH: usize = 10;
/// Audio arrives base64-encoded inside the server-fn payload.
const MAX_BODY_BYTES: usize = 10 * 1024 * 1024;

fn http_client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(reqwest::Client::new)
}

/// Entry point for the server binary: serves the CSR bundle (`PUBLIC_DIR`,
/// default `./public`) with an SPA fallback, registers server functions, and
/// binds to `IP`/`PORT` (Cloud Run contract).
pub fn main() {
    use axum::extract::DefaultBodyLimit;
    use axum::routing::get;
    use dioxus::server::{DioxusRouterExt, ServeConfig};

    if std::env::var("IP").is_err() {
        std::env::set_var("IP", "0.0.0.0");
    }

    dioxus::serve(|| async {
        let router = axum::Router::new()
            .route("/health", get(|| async { "ok" }))
            .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
            .serve_dioxus_application(ServeConfig::new(), crate::App);

        Ok(router)
    })
}

/// Validates the problem and forwards it to the agent (`${AGENT_URL}/solve`),
/// passing the trail JSON through untouched — the agent's shape is the
/// contract the client renders.
pub async fn solve_impl(problem: &str) -> Result<Value, String> {
    if problem.trim().chars().count() < MIN_PROBLEM_LENGTH {
        return Err(format!(
            "problem must be at least {MIN_PROBLEM_LENGTH} characters"
        ));
    }
    let agent_url = std::env::var("AGENT_URL").unwrap_or_else(|_| "http://localhost:8000".into());

    let response = http_client()
        .post(format!("{agent_url}/solve"))
        .json(&json!({ "problem": problem }))
        .timeout(AGENT_TIMEOUT)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|err| {
            tracing::error!("Agent call failed: {err}");
            "Agent call failed".to_string()
        })?;

    response.json().await.map_err(|err| {
        tracing::error!("Agent call failed: {err}");
        "Agent call failed".to_string()
    })
}

/// Sends base64 audio to Google Cloud Speech-to-Text and returns the joined
/// transcript. Ported verbatim from the old api crate's speech route.
pub async fn transcribe_impl(audio_content: &str, mime_type: &str) -> Result<String, String> {
    if audio_content.is_empty() {
        return Err("audioContent should not be empty".into());
    }
    if mime_type.is_empty() {
        return Err("mimeType should not be empty".into());
    }
    let encoding = encoding_from_mime_type(mime_type)?;
    let token = get_access_token().await?;

    let response = http_client()
        .post(SPEECH_API_URL)
        .bearer_auth(token)
        .json(&json!({
            "config": {
                "encoding": encoding,
                "languageCode": "pl-PL",
                "alternativeLanguageCodes": ["en-US"],
                "enableAutomaticPunctuation": true,
                "model": "latest_short",
            },
            "audio": { "content": audio_content },
        }))
        .send()
        .await
        .map_err(|err| {
            tracing::error!("Speech-to-Text request failed: {err}");
            "Speech-to-Text request failed".to_string()
        })?;

    let status = response.status();
    let data: Value = response.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        let detail = data
            .pointer("/error/message")
            .and_then(Value::as_str)
            .unwrap_or_else(|| status.as_str());
        tracing::error!("Speech-to-Text request failed: {detail}");
        return Err("Speech-to-Text request failed".into());
    }

    Ok(extract_transcript(&data))
}

/// Maps a browser MediaRecorder MIME type onto a Speech-to-Text encoding.
fn encoding_from_mime_type(mime_type: &str) -> Result<&'static str, String> {
    if mime_type.contains("webm") {
        Ok("WEBM_OPUS")
    } else if mime_type.contains("ogg") {
        Ok("OGG_OPUS")
    } else if mime_type.contains("mpeg") || mime_type.contains("mp3") {
        Ok("MP3")
    } else {
        Err(format!("Unsupported audio MIME type: {mime_type}"))
    }
}

/// Joins every non-empty `results[].alternatives[].transcript` with a space.
fn extract_transcript(data: &Value) -> String {
    data.pointer("/results")
        .and_then(Value::as_array)
        .map(|results| {
            results
                .iter()
                .filter_map(|result| result.get("alternatives").and_then(Value::as_array))
                .flatten()
                .filter_map(|alternative| alternative.get("transcript").and_then(Value::as_str))
                .map(str::trim)
                .filter(|transcript| !transcript.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// `GOOGLE_CLOUD_ACCESS_TOKEN` env override, else the GCP metadata server
/// (Cloud Run service account).
async fn get_access_token() -> Result<String, String> {
    if let Ok(token) = std::env::var("GOOGLE_CLOUD_ACCESS_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }
    let response = http_client()
        .get(METADATA_TOKEN_URL)
        .header("Metadata-Flavor", "Google")
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .ok()
        .filter(|response| response.status().is_success());

    let Some(response) = response else {
        tracing::error!("Could not obtain Google Cloud access token from metadata server");
        return Err("Google Cloud authentication is not configured".into());
    };
    let data: Value = response.json().await.unwrap_or(Value::Null);
    data.get("access_token")
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| "Google Cloud metadata server returned no access token".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_supported_mime_types_to_encodings() {
        assert_eq!(
            encoding_from_mime_type("audio/webm;codecs=opus").unwrap(),
            "WEBM_OPUS"
        );
        assert_eq!(encoding_from_mime_type("audio/ogg").unwrap(), "OGG_OPUS");
        assert_eq!(encoding_from_mime_type("audio/mpeg").unwrap(), "MP3");
        assert!(encoding_from_mime_type("audio/flac").is_err());
    }

    #[test]
    fn joins_transcripts_across_results() {
        let data = serde_json::json!({
            "results": [
                { "alternatives": [{ "transcript": " Hello " }, { "transcript": "" }] },
                { "alternatives": [{ "transcript": "world" }] },
            ]
        });
        assert_eq!(extract_transcript(&data), "Hello world");
    }

    #[tokio::test]
    async fn solve_rejects_short_problems() {
        let err = solve_impl("too short").await.unwrap_err();
        assert!(err.contains("at least 10 characters"));
    }

    #[tokio::test]
    async fn solve_passes_agent_response_through() {
        // Mock agent: echoes a canned trail.
        let router = axum::Router::new().route(
            "/solve",
            axum::routing::post(|| async {
                axum::Json(serde_json::json!({ "step5_choice": { "winner_id": "triz-1" } }))
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        std::env::set_var("AGENT_URL", format!("http://{addr}"));

        let trail = solve_impl("a sufficiently long problem statement")
            .await
            .unwrap();
        assert_eq!(trail["step5_choice"]["winner_id"], "triz-1");
    }

    #[tokio::test]
    async fn transcribe_rejects_empty_audio_and_bad_mime() {
        assert!(transcribe_impl("", "audio/webm").await.is_err());
        assert!(transcribe_impl("Zm9v", "audio/flac")
            .await
            .unwrap_err()
            .contains("Unsupported"));
    }
}
