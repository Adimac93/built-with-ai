mod error;
mod routes;

use std::net::SocketAddr;

use axum::extract::DefaultBodyLimit;
use axum::http::{header, HeaderValue, Method};
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;

/// Matches the Express `json({ limit: '10mb' })` body limit — audio uploads
/// arrive as base64 JSON, not multipart.
const MAX_BODY_BYTES: usize = 10 * 1024 * 1024;

/// Origins allowed by the Nest API's CORS config, plus the Dioxus dev
/// server (`dx serve`, port 8080) which replaced Angular's :4200.
const ALLOWED_ORIGINS: [&str; 4] = [
    "http://localhost:4200",
    "http://localhost:8080",
    "https://frontend-1062481454649.us-central1.run.app",
    "https://ai.sidequestly.xyz",
];

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub agent_url: String,
}

/// Builds the router with every route mounted under the `/api` prefix,
/// mirroring Nest's `setGlobalPrefix('api')`.
fn app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(ALLOWED_ORIGINS.map(HeaderValue::from_static))
        .allow_methods([
            Method::GET,
            Method::HEAD,
            Method::PUT,
            Method::PATCH,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/api", get(routes::health::get_data))
        .route("/api/solve", post(routes::solve::solve))
        .route("/api/speech/transcribe", post(routes::speech::transcribe))
        .layer(cors)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let state = AppState {
        client: reqwest::Client::new(),
        agent_url: std::env::var("AGENT_URL").unwrap_or_else(|_| "http://localhost:8000".into()),
    };

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {addr}: {err}"));

    tracing::info!("🚀 Application is running on: http://0.0.0.0:{port}/api");
    axum::serve(listener, app(state)).await.expect("server error");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use serde_json::{json, Value};
    use tower::ServiceExt;

    fn test_app() -> Router {
        app(AppState {
            client: reqwest::Client::new(),
            agent_url: "http://localhost:8000".into(),
        })
    }

    async fn body_json(body: Body) -> Value {
        let bytes = to_bytes(body, usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn health_check_returns_hello_api() {
        let response = test_app()
            .oneshot(Request::get("/api").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            body_json(response.into_body()).await,
            json!({ "message": "Hello API" })
        );
    }

    #[tokio::test]
    async fn solve_rejects_short_problem_with_nest_shaped_400() {
        let response = test_app()
            .oneshot(
                Request::post("/api/solve")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"problem":"short"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            body_json(response.into_body()).await,
            json!({
                "message": ["problem must be longer than or equal to 10 characters"],
                "error": "Bad Request",
                "statusCode": 400,
            })
        );
    }

    #[tokio::test]
    async fn transcribe_rejects_unsupported_mime_type() {
        let response = test_app()
            .oneshot(
                Request::post("/api/speech/transcribe")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"audioContent":"Zm9v","mimeType":"audio/flac"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            body_json(response.into_body()).await,
            json!({
                "message": "Unsupported audio MIME type: audio/flac",
                "error": "Bad Request",
                "statusCode": 400,
            })
        );
    }

    #[tokio::test]
    async fn cors_preflight_allows_frontend_origin() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/api/solve")
                    .header("Origin", "http://localhost:4200")
                    .header("Access-Control-Request-Method", "POST")
                    .header("Access-Control-Request-Headers", "content-type")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let headers = response.headers();
        assert_eq!(
            headers.get("access-control-allow-origin").unwrap(),
            "http://localhost:4200"
        );
        assert_eq!(
            headers.get("access-control-allow-credentials").unwrap(),
            "true"
        );
        assert!(headers
            .get("access-control-allow-headers")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("content-type"));
    }
}
