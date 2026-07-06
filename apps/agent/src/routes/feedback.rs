use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};

/// Mirrors the Python `Feedback` pydantic model (`app_utils/typing.py`).
#[derive(Deserialize)]
pub struct Feedback {
    pub score: f64,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
}

/// `POST /feedback` — emits one structured-JSON log line to stdout. Cloud Run
/// forwards it to Cloud Logging as `jsonPayload`, which keeps the BigQuery
/// feedback sink working (its filter matches `jsonPayload.log_type="feedback"
/// jsonPayload.service_name="agent"` — see
/// `deployment/terraform/single-project/telemetry.tf`).
pub async fn feedback(Json(body): Json<Feedback>) -> Json<Value> {
    println!("{}", feedback_log_line(&body));
    Json(json!({ "status": "success" }))
}

fn feedback_log_line(feedback: &Feedback) -> String {
    json!({
        "severity": "INFO",
        "message": "feedback",
        "log_type": "feedback",
        "service_name": "agent",
        "score": feedback.score,
        "text": feedback.text.as_deref().unwrap_or(""),
        "user_id": feedback.user_id,
        "session_id": feedback.session_id,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_line_matches_bigquery_sink_filter() {
        let line = feedback_log_line(&Feedback {
            score: 4.5,
            text: Some("great".into()),
            user_id: Some("u1".into()),
            session_id: None,
        });
        let parsed: Value = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed["log_type"], "feedback");
        assert_eq!(parsed["service_name"], "agent");
        assert_eq!(parsed["severity"], "INFO");
        assert_eq!(parsed["score"], 4.5);
        assert_eq!(parsed["text"], "great");
        assert!(!line.contains('\n'));
    }
}
