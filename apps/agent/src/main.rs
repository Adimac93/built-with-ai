mod error;
mod gemini;
mod pipeline;
mod routes;
mod triz;

use std::net::SocketAddr;

use axum::http::{header, HeaderValue};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};

use gemini::GeminiClient;

#[derive(Clone)]
pub struct AppState {
    pub gemini: GeminiClient,
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Router with the Python service's surface: `POST /solve`, `POST /feedback`,
/// plus a plain health root. CORS only when `ALLOW_ORIGINS` is set (FastAPI
/// received `allow_origins=None` otherwise).
fn app(state: AppState, allow_origins: Option<&str>) -> Router {
    let mut router = Router::new()
        .route("/", get(health))
        .route("/solve", post(routes::solve::solve))
        .route("/feedback", post(routes::feedback::feedback));

    if let Some(origins) = allow_origins {
        let origins: Vec<HeaderValue> = origins
            .split(',')
            .filter_map(|origin| origin.trim().parse().ok())
            .collect();
        router = router.layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_methods(Any)
                .allow_headers([header::CONTENT_TYPE]),
        );
    }

    router.with_state(state)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let state = AppState {
        gemini: GeminiClient::from_env(),
    };
    let allow_origins = std::env::var("ALLOW_ORIGINS").ok();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {addr}: {err}"));

    tracing::info!("Agent listening on http://0.0.0.0:{port}");
    axum::serve(listener, app(state, allow_origins.as_deref()))
        .await
        .expect("server error");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::extract::Request as ExtractRequest;
    use axum::http::{Request, StatusCode};
    use gemini::GeminiAuth;
    use tower::ServiceExt;

    /// Mock Gemini backend: discriminates pipeline steps by prompt markers
    /// and returns canned structured responses in the generateContent shape.
    async fn mock_gemini(request: ExtractRequest) -> Json<Value> {
        let bytes = axum::body::to_bytes(request.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        let user_text = body
            .pointer("/contents/0/parts/0/text")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let system_text = body
            .pointer("/systemInstruction/parts/0/text")
            .and_then(Value::as_str)
            .unwrap_or_default();

        let text = if system_text.contains("technical problem analyst") {
            "Normalized: machine overheats under load.".to_string()
        } else if user_text.contains("You are a TRIZ expert") {
            json!({ "improving_param": 21, "worsening_param": 17, "justification": "power vs temperature" }).to_string()
        } else if user_text.contains("Extract evaluation criteria") {
            json!({
                "generic_criteria": ["Resolves the technical contradiction", "Technical feasibility", "Cost and scalability"],
                "problem_specific_criteria": ["Effective under load"],
            }).to_string()
        } else if user_text.contains("TRIZ solution inventor") {
            json!({ "candidates": [
                { "principle_number": 35, "principle_name": "Parameter changes", "idea": "use PCM", "trace_id": "triz-35" },
            ]}).to_string()
        } else if user_text.contains("SCAMPER method") {
            json!({ "candidates": [
                { "operator": "S", "operator_name": "Substitute", "idea": "liquid metal coolant", "trace_id": "scamper-S" },
                { "operator": "C", "operator_name": "Combine", "idea": "combine TEC", "trace_id": "scamper-C" },
                { "operator": "E", "operator_name": "Eliminate", "idea": "remove flow restrictions", "trace_id": "scamper-E" },
            ]}).to_string()
        } else if user_text.contains("objective solution evaluator") {
            json!({ "evaluations": [
                { "trace_id": "triz-35", "scores": { "Technical feasibility": 90 }, "justification": "solid", "total": 90 },
                { "trace_id": "scamper-S", "scores": { "Technical feasibility": 80 }, "justification": "ok", "total": 80 },
            ]}).to_string()
        } else {
            panic!("mock Gemini got unrecognized prompt: {user_text}");
        };

        Json(json!({ "candidates": [{ "content": { "parts": [{ "text": text }] } }] }))
    }

    async fn spawn_mock_gemini() -> String {
        let router = Router::new().fallback(post(mock_gemini));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        format!("http://{addr}")
    }

    async fn body_json(body: Body) -> Value {
        let bytes = to_bytes(body, usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let state = AppState {
            gemini: GeminiClient::new("http://unused".into(), GeminiAuth::ApiKey("test".into())),
        };
        let response = app(state, None)
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_json(response.into_body()).await, json!({ "status": "ok" }));
    }

    #[tokio::test]
    async fn solve_returns_full_trail_via_mock_gemini() {
        let base_url = spawn_mock_gemini().await;
        let state = AppState {
            gemini: GeminiClient::new(base_url, GeminiAuth::ApiKey("test".into())),
        };

        let response = app(state, None)
            .oneshot(
                Request::post("/solve")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"problem":"my machine overheats under load"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let trail = body_json(response.into_body()).await;

        for key in [
            "step1_problem",
            "step2_contradiction",
            "step2a_lookup",
            "step1a_criteria",
            "step3a_triz_candidates",
            "step3b_scamper_candidates",
            "step4_evaluation",
            "step5_choice",
        ] {
            assert!(trail.get(key).is_some(), "trail missing {key}");
        }

        assert_eq!(trail["step2_contradiction"]["improving_param"], 21);
        assert_eq!(trail["step2a_lookup"]["improving_name"], "Power");
        assert_eq!(trail["step2a_lookup"]["worsening_name"], "Temperature");
        let principles = trail["step2a_lookup"]["principles"].as_array().unwrap();
        assert!(!principles.is_empty());
        assert!(principles.iter().all(|p| p["number"].is_string()));
        assert_eq!(trail["step5_choice"]["winner_id"], "triz-35");
        assert_eq!(trail["step5_choice"]["winner_total"], 90);
    }

    #[tokio::test]
    async fn feedback_returns_success() {
        let state = AppState {
            gemini: GeminiClient::new("http://unused".into(), GeminiAuth::ApiKey("test".into())),
        };
        let response = app(state, None)
            .oneshot(
                Request::post("/feedback")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"score": 5, "text": "nice"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_json(response.into_body()).await, json!({ "status": "success" }));
    }

    #[tokio::test]
    async fn cors_layer_active_when_allow_origins_set() {
        let state = AppState {
            gemini: GeminiClient::new("http://unused".into(), GeminiAuth::ApiKey("test".into())),
        };
        let response = app(state, Some("http://localhost:4200"))
            .oneshot(
                Request::builder()
                    .method("OPTIONS")
                    .uri("/solve")
                    .header("Origin", "http://localhost:4200")
                    .header("Access-Control-Request-Method", "POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response.headers().get("access-control-allow-origin").unwrap(),
            "http://localhost:4200"
        );
    }
}
