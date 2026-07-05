//! HTTP calls to the Heureka API (Angular: AnalysisEngine + SpeechTranscriptionService).

use dioxus::prelude::ReadableExt;
use serde_json::json;

use crate::state::API_URL;
use crate::trail::{map_to_trail, EvalMode, SolveApiResponse, Trail};

/// `POST /solve` and map the response to the rendered trail.
pub async fn solve(problem: &str, eval_mode: EvalMode) -> Result<Trail, String> {
    let url = format!("{}/solve", API_URL.read().clone());
    let response = reqwest::Client::new()
        .post(url)
        .json(&json!({ "problem": problem }))
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|err| err.to_string())?
        .json::<SolveApiResponse>()
        .await
        .map_err(|err| err.to_string())?;

    map_to_trail(&response, eval_mode).ok_or_else(|| "empty trail".to_string())
}

/// `POST /speech/transcribe` — base64 audio in, trimmed transcript out.
pub async fn transcribe(audio_content: &str, mime_type: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct TranscribeResponse {
        #[serde(default)]
        text: String,
    }
    let url = format!("{}/speech/transcribe", API_URL.read().clone());
    let response = reqwest::Client::new()
        .post(url)
        .json(&json!({
            "audioContent": audio_content,
            "mimeType": mime_type,
            "languageCode": "pl-PL",
        }))
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|err| err.to_string())?
        .json::<TranscribeResponse>()
        .await
        .map_err(|err| err.to_string())?;

    Ok(response.text.trim().to_string())
}
