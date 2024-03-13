use axum::{ http::StatusCode, response::{ IntoResponse, Response }, Json };
use serde::Serialize;
use serde_json::json;

use super::security::auth_error::AuthError;

#[derive(Debug, Clone, Serialize)]
pub enum ApiErrorType {
    Auth(AuthError),
    Api,
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub error_type: ApiErrorType,
    pub error_message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{status_code: {}, error_message: {}}}", self.status_code, self.error_message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {}", self.to_string());

        (
            self.status_code,
            Json(json!({"type": self.error_type,"message": self.error_message})),
        ).into_response()
    }
}

impl From<StatusCode> for ApiError {
    fn from(status_code: StatusCode) -> Self {
        ApiError {
            status_code,
            error_type: ApiErrorType::Api,
            error_message: status_code.to_string(),
        }
    }
}
