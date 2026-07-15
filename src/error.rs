use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderValue;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("payload too large: {0}")]
    PayloadTooLarge(String),

    #[error("too many requests: {0}")]
    TooManyRequests(String),

    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, detail) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.clone()),
            ApiError::PayloadTooLarge(message) => {
                (StatusCode::PAYLOAD_TOO_LARGE, message.clone())
            }
            ApiError::TooManyRequests(message) => {
                (StatusCode::TOO_MANY_REQUESTS, message.clone())
            }
            ApiError::Sqlx(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "not found".to_string())
            }
            other => {
                tracing::error!(error = ?other, "api error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        };
        let title = status.canonical_reason().unwrap_or("Error");
        let mut response = (
            status,
            Json(json!({
                "type": "about:blank",
                "title": title,
                "status": status.as_u16(),
                "detail": detail,
                "error": detail,
            })),
        )
            .into_response();
        response.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );
        response
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
