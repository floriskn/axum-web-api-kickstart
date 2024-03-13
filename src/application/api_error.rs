use axum::{
    extract::{ path::ErrorKind, rejection::{ JsonRejection, PathRejection } },
    http::StatusCode,
    response::{ IntoResponse, Response },
    Json,
};
use serde::Serialize;
use serde_json::json;

use super::{ api_path::PathError, security::auth_error::AuthError };

#[derive(Debug, Clone, Serialize)]
pub enum ApiErrorType {
    Auth(AuthError),
    Api,
    Path(PathError),
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

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        ApiError {
            status_code: rejection.status(),
            error_type: ApiErrorType::Api,
            error_message: rejection.body_text(),
        }
    }
}

impl From<PathRejection> for ApiError {
    fn from(rejection: PathRejection) -> Self {
        let (status_code, error_type, error_message) = match rejection {
            PathRejection::FailedToDeserializePathParams(inner) => {
                let kind = inner.into_kind();
                match &kind {
                    ErrorKind::WrongNumberOfParameters { .. } =>
                        (
                            StatusCode::BAD_REQUEST,
                            PathError::WrongNumberOfParameters,
                            kind.to_string(),
                        ),

                    ErrorKind::ParseErrorAtKey { key, .. } =>
                        (
                            StatusCode::BAD_REQUEST,
                            PathError::ParseErrorAtKey(key.clone()),
                            kind.to_string(),
                        ),

                    ErrorKind::ParseErrorAtIndex { index, .. } =>
                        (
                            StatusCode::BAD_REQUEST,
                            PathError::ParseErrorAtIndex(*index),
                            kind.to_string(),
                        ),

                    ErrorKind::ParseError { .. } =>
                        (StatusCode::BAD_REQUEST, PathError::ParseError, kind.to_string()),

                    ErrorKind::InvalidUtf8InPathParam { key } =>
                        (
                            StatusCode::BAD_REQUEST,
                            PathError::InvalidUtf8InPathParam(key.clone()),
                            kind.to_string(),
                        ),

                    ErrorKind::UnsupportedType { .. } => {
                        // this error is caused by the programmer using an unsupported type
                        // (such as nested maps) so respond with `500` instead
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            PathError::UnsupportedType,
                            kind.to_string(),
                        )
                    }

                    ErrorKind::Message(msg) =>
                        (StatusCode::BAD_REQUEST, PathError::Message, msg.clone()),

                    _ =>
                        (
                            StatusCode::BAD_REQUEST,
                            PathError::UnhandledDeserialization,
                            format!("Unhandled deserialization error: {kind}"),
                        ),
                }
            }
            PathRejection::MissingPathParams(error) =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    PathError::MissingPathParams,
                    error.to_string(),
                ),
            _ =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    PathError::UnhandledRejection,
                    format!("Unhandled path rejection: {rejection}"),
                ),
        };
        ApiError {
            status_code,
            error_type: ApiErrorType::Path(error_type),
            error_message,
        }
    }
}
