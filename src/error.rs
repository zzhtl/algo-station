use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m.clone()),
            ApiError::Sqlx(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "not found".to_string())
            }
            other => {
                tracing::error!(error = ?other, "api error");
                (StatusCode::INTERNAL_SERVER_ERROR, other.to_string())
            }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
