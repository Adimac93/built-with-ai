//! Client-facing API calls. In fullstack builds these are Dioxus server
//! functions, so the browser talks to the frontend server directly instead of
//! a separate `apps/api` service.

use dioxus::prelude::*;

use crate::trail::{map_to_trail, EvalMode, SolveApiResponse, Trail};

#[server(endpoint = "/api/solve")]
async fn solve_server(problem: String) -> ServerFnResult<serde_json::Value> {
    crate::server::solve_impl(&problem)
        .await
        .map_err(ServerFnError::new)
}

#[server(endpoint = "/api/speech/transcribe")]
async fn transcribe_server(audio_content: String, mime_type: String) -> ServerFnResult<String> {
    crate::server::transcribe_impl(&audio_content, &mime_type)
        .await
        .map_err(ServerFnError::new)
}

/// Runs the agent and maps the response to the rendered trail.
pub async fn solve(problem: &str, eval_mode: EvalMode) -> Result<Trail, String> {
    let response = solve_server(problem.to_string())
        .await
        .map_err(|err| err.to_string())?;
    let response: SolveApiResponse =
        serde_json::from_value(response).map_err(|err| err.to_string())?;

    map_to_trail(&response, eval_mode).ok_or_else(|| "empty trail".to_string())
}

/// Base64 audio in, trimmed transcript out.
pub async fn transcribe(audio_content: &str, mime_type: &str) -> Result<String, String> {
    let response = transcribe_server(audio_content.to_string(), mime_type.to_string())
        .await
        .map_err(|err| err.to_string())?;

    Ok(response.trim().to_string())
}
