use axum::Json;
use serde_json::{json, Value};

/// `GET /api` — health check; the `api-e2e` suite asserts this exact payload.
pub async fn get_data() -> Json<Value> {
    Json(json!({ "message": "Hello API" }))
}
