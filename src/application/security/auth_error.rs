use crate::application::api_error::{ ApiError, ApiErrorType };
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    ExpiredToken,
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        let (status_code, error_message) = match err {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::BAD_REQUEST, "Expired token"),
        };
        ApiError {
            status_code,
            error_type: ApiErrorType::Auth(err),
            error_message: error_message.to_owned(),
        }
    }
}
