use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// API error that serializes to the NestJS exception shape
/// (`{ "message": ..., "error": ..., "statusCode": ... }`) so the
/// frontend sees identical error payloads after the Rust rewrite.
#[derive(Debug, PartialEq)]
pub enum ApiError {
    /// 400 with an array `message`, mirroring class-validator output.
    Validation(Vec<String>),
    /// 400 with a string `message` (Nest `BadRequestException`).
    BadRequest(String),
    /// 500 with a string `message` (Nest `InternalServerErrorException`).
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message, error) = match self {
            ApiError::Validation(messages) => {
                (StatusCode::BAD_REQUEST, json!(messages), "Bad Request")
            }
            ApiError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, json!(message), "Bad Request")
            }
            ApiError::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(message),
                "Internal Server Error",
            ),
        };

        let body = Json(json!({
            "message": message,
            "error": error,
            "statusCode": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn body_json(error: ApiError) -> (StatusCode, serde_json::Value) {
        let response = error.into_response();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap())
    }

    #[tokio::test]
    async fn validation_error_has_nest_shape_with_message_array() {
        let (status, body) = body_json(ApiError::Validation(vec!["oops".into()])).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(
            body,
            json!({ "message": ["oops"], "error": "Bad Request", "statusCode": 400 })
        );
    }

    #[tokio::test]
    async fn internal_error_has_nest_shape_with_message_string() {
        let (status, body) = body_json(ApiError::Internal("Agent call failed".into())).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            body,
            json!({ "message": "Agent call failed", "error": "Internal Server Error", "statusCode": 500 })
        );
    }
}
