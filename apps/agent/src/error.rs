use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Error serialized as `{ "detail": ... }`, matching the FastAPI error shape
/// the previous Python service produced.
#[derive(Debug)]
pub struct AgentError {
    pub status: StatusCode,
    pub detail: String,
}

impl AgentError {
    pub fn internal(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            detail: detail.into(),
        }
    }
}

impl IntoResponse for AgentError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({ "detail": self.detail }))).into_response()
    }
}
