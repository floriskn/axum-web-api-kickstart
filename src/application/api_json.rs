use axum::{
    async_trait,
    extract::{ rejection::JsonRejection, FromRequest, Request },
    response::{ IntoResponse, Response },
};

use super::api_error::ApiError;

pub struct Json<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S>
    for Json<T>
    where axum::Json<T>: FromRequest<S, Rejection = JsonRejection>, S: Send + Sync
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}

impl<T> IntoResponse for Json<T> where axum::Json<T>: IntoResponse {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
