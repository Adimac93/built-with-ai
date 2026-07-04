use std::time::Duration;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;
use crate::AppState;

/// Matches the Nest `@MinLength(10)` constraint on `problem`.
const MIN_PROBLEM_LENGTH: usize = 10;
/// Matches the 120s axios timeout used for the agent call in the Nest API.
const AGENT_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Deserialize)]
pub struct SolveRequest {
    #[serde(default)]
    problem: Option<String>,
}

/// `POST /api/solve` — validates the problem statement and forwards it to the
/// Python ADK agent (`${AGENT_URL}/solve`), passing the agent's JSON response
/// through untouched. The agent's shape is authoritative; the frontend depends
/// on it as-is, so no response model is imposed here.
pub async fn solve(
    State(state): State<AppState>,
    Json(body): Json<SolveRequest>,
) -> Result<Json<Value>, ApiError> {
    let problem = validate_problem(body.problem.as_deref())?;

    let response = state
        .client
        .post(format!("{}/solve", state.agent_url))
        .json(&serde_json::json!({ "problem": problem }))
        .timeout(AGENT_TIMEOUT)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|err| {
            tracing::error!("Agent call failed: {err}");
            ApiError::Internal("Agent call failed".into())
        })?;

    let trail: Value = response.json().await.map_err(|err| {
        tracing::error!("Agent call failed: {err}");
        ApiError::Internal("Agent call failed".into())
    })?;

    Ok(Json(trail))
}

/// Replicates the class-validator messages for `@IsNotEmpty` + `@MinLength(10)`.
fn validate_problem(problem: Option<&str>) -> Result<&str, ApiError> {
    let mut errors = Vec::new();

    let problem = problem.unwrap_or_default();
    if problem.chars().count() < MIN_PROBLEM_LENGTH {
        errors.push(format!(
            "problem must be longer than or equal to {MIN_PROBLEM_LENGTH} characters"
        ));
    }
    if problem.is_empty() {
        errors.push("problem should not be empty".to_string());
    }

    if errors.is_empty() {
        Ok(problem)
    } else {
        Err(ApiError::Validation(errors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_problem() {
        let err = validate_problem(None).unwrap_err();
        assert_eq!(
            err,
            ApiError::Validation(vec![
                "problem must be longer than or equal to 10 characters".into(),
                "problem should not be empty".into(),
            ])
        );
    }

    #[test]
    fn rejects_problem_shorter_than_10_chars() {
        let err = validate_problem(Some("too short")).unwrap_err();
        assert_eq!(
            err,
            ApiError::Validation(vec![
                "problem must be longer than or equal to 10 characters".into()
            ])
        );
    }

    #[test]
    fn accepts_problem_of_exactly_10_chars() {
        assert_eq!(validate_problem(Some("exactly 10")).unwrap(), "exactly 10");
    }

    #[test]
    fn counts_characters_not_bytes() {
        // 10 multi-byte characters must pass, mirroring JS string length.
        assert!(validate_problem(Some("zażółćgęśl")).is_ok());
    }
}
