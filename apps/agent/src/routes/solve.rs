use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::Value;

use crate::error::AgentError;
use crate::pipeline;
use crate::AppState;

#[derive(Deserialize)]
pub struct SolveRequest {
    pub problem: String,
}

/// `POST /solve` — runs the TRIZ+SCAMPER pipeline and returns the reasoning
/// trail verbatim. Stateless per request, like the Python version (which
/// created and discarded an in-memory ADK session each call).
pub async fn solve(
    State(state): State<AppState>,
    Json(body): Json<SolveRequest>,
) -> Result<Json<Value>, AgentError> {
    let trail = pipeline::run(&state.gemini, &body.problem).await?;
    Ok(Json(trail))
}
