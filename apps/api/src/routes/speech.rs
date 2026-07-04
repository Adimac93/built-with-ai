use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::AppState;

const SPEECH_API_URL: &str = "https://speech.googleapis.com/v1/speech:recognize";
const METADATA_TOKEN_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
/// Matches the Nest `@IsIn` constraint on `encoding`.
const ALLOWED_ENCODINGS: [&str; 4] = ["WEBM_OPUS", "OGG_OPUS", "LINEAR16", "MP3"];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscribeRequest {
    #[serde(default)]
    audio_content: Option<String>,
    #[serde(default)]
    mime_type: Option<String>,
    language_code: Option<String>,
    encoding: Option<String>,
}

/// `POST /api/speech/transcribe` — sends base64 audio to Google Cloud
/// Speech-to-Text and returns the joined transcript as `{ "text": ... }`.
pub async fn transcribe(
    State(state): State<AppState>,
    Json(body): Json<TranscribeRequest>,
) -> Result<Json<Value>, ApiError> {
    let (audio_content, mime_type) = validate_request(&body)?;
    let encoding = match body.encoding.as_deref() {
        Some(encoding) => encoding,
        None => encoding_from_mime_type(mime_type)?,
    };

    let token = get_access_token(&state.client).await?;

    let response = state
        .client
        .post(SPEECH_API_URL)
        .bearer_auth(token)
        .json(&json!({
            "config": {
                "encoding": encoding,
                "languageCode": body.language_code.as_deref().unwrap_or("pl-PL"),
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
            ApiError::Internal("Speech-to-Text request failed".into())
        })?;

    let status = response.status();
    let data: Value = response.json().await.unwrap_or(Value::Null);

    if !status.is_success() {
        let detail = data
            .pointer("/error/message")
            .and_then(Value::as_str)
            .unwrap_or_else(|| status.as_str());
        tracing::error!("Speech-to-Text request failed: {detail}");
        return Err(ApiError::Internal("Speech-to-Text request failed".into()));
    }

    Ok(Json(json!({ "text": extract_transcript(&data) })))
}

/// Replicates the class-validator constraints: `audioContent` and `mimeType`
/// are required and non-empty, `encoding` (when present) must be in the
/// allowed set.
fn validate_request(body: &TranscribeRequest) -> Result<(&str, &str), ApiError> {
    let mut errors = Vec::new();

    let audio_content = body.audio_content.as_deref().unwrap_or_default();
    if audio_content.is_empty() {
        errors.push("audioContent should not be empty".to_string());
    }

    let mime_type = body.mime_type.as_deref().unwrap_or_default();
    if mime_type.is_empty() {
        errors.push("mimeType should not be empty".to_string());
    }

    if let Some(encoding) = body.encoding.as_deref() {
        if !ALLOWED_ENCODINGS.contains(&encoding) {
            errors.push(format!(
                "encoding must be one of the following values: {}",
                ALLOWED_ENCODINGS.join(", ")
            ));
        }
    }

    if errors.is_empty() {
        Ok((audio_content, mime_type))
    } else {
        Err(ApiError::Validation(errors))
    }
}

/// Maps a browser MediaRecorder MIME type onto a Speech-to-Text encoding,
/// mirroring `SpeechService.encodingFromMimeType` in the Nest API.
fn encoding_from_mime_type(mime_type: &str) -> Result<&'static str, ApiError> {
    if mime_type.contains("webm") {
        Ok("WEBM_OPUS")
    } else if mime_type.contains("ogg") {
        Ok("OGG_OPUS")
    } else if mime_type.contains("mpeg") || mime_type.contains("mp3") {
        Ok("MP3")
    } else {
        Err(ApiError::BadRequest(format!(
            "Unsupported audio MIME type: {mime_type}"
        )))
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

/// Uses `GOOGLE_CLOUD_ACCESS_TOKEN` when set (local development), otherwise
/// mints an OAuth token from the GCP metadata server (Cloud Run service
/// account).
async fn get_access_token(client: &reqwest::Client) -> Result<String, ApiError> {
    if let Ok(token) = std::env::var("GOOGLE_CLOUD_ACCESS_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    let response = client
        .get(METADATA_TOKEN_URL)
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .ok()
        .filter(|response| response.status().is_success());

    let Some(response) = response else {
        tracing::error!("Could not obtain Google Cloud access token from metadata server");
        return Err(ApiError::Internal(
            "Google Cloud authentication is not configured".into(),
        ));
    };

    let data: Value = response.json().await.unwrap_or(Value::Null);
    data.get("access_token")
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| {
            ApiError::Internal("Google Cloud metadata server returned no access token".into())
        })
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
        assert_eq!(encoding_from_mime_type("audio/mp3").unwrap(), "MP3");
    }

    #[test]
    fn rejects_unsupported_mime_type() {
        assert_eq!(
            encoding_from_mime_type("audio/flac").unwrap_err(),
            ApiError::BadRequest("Unsupported audio MIME type: audio/flac".into())
        );
    }

    #[test]
    fn rejects_encoding_outside_allowed_set() {
        let body = TranscribeRequest {
            audio_content: Some("Zm9v".into()),
            mime_type: Some("audio/webm".into()),
            language_code: None,
            encoding: Some("FLAC".into()),
        };
        assert_eq!(
            validate_request(&body).unwrap_err(),
            ApiError::Validation(vec![
                "encoding must be one of the following values: WEBM_OPUS, OGG_OPUS, LINEAR16, MP3"
                    .into()
            ])
        );
    }

    #[test]
    fn requires_audio_content_and_mime_type() {
        let body = TranscribeRequest {
            audio_content: None,
            mime_type: None,
            language_code: None,
            encoding: None,
        };
        assert_eq!(
            validate_request(&body).unwrap_err(),
            ApiError::Validation(vec![
                "audioContent should not be empty".into(),
                "mimeType should not be empty".into(),
            ])
        );
    }

    #[test]
    fn joins_transcripts_across_results_and_alternatives() {
        let data = json!({
            "results": [
                { "alternatives": [{ "transcript": " Hello " }, { "transcript": "" }] },
                { "alternatives": [{ "transcript": "world" }] },
                {},
            ]
        });
        assert_eq!(extract_transcript(&data), "Hello world");
    }

    #[test]
    fn empty_response_yields_empty_text() {
        assert_eq!(extract_transcript(&json!({})), "");
    }
}
