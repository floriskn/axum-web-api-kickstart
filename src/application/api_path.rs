use axum::{ async_trait, extract::FromRequestParts, http::request::Parts };
use serde::{ de::DeserializeOwned, Serialize };

use super::api_error::ApiError;

#[derive(Debug, Clone, Serialize)]
pub enum PathError {
    WrongNumberOfParameters,
    ParseErrorAtKey(String),
    ParseErrorAtIndex(usize),
    ParseError,
    InvalidUtf8InPathParam(String),
    UnsupportedType,
    Message,
    MissingPathParams,
    UnhandledDeserialization,
    UnhandledRejection,
}

pub struct Path<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S>
    for Path<T>
    where
        // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
        T: DeserializeOwned + Send,
        S: Send + Sync
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}
