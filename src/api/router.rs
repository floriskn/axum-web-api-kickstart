use axum::{
    body::Body,
    extract::Request,
    http::{ Method, StatusCode },
    middleware::Next,
    response::{ IntoResponse, Response },
    routing::get,
    Router,
};
use std::collections::HashMap;

use super::{ auth, users };

use crate::application::{
    api_error::{ ApiError, ApiErrorType },
    api_json::Json,
    api_path::Path,
    app_const::*,
    state::SharedState,
};

pub fn routes(state: SharedState) -> Router {
    // build the service routes
    Router::new()
        .route("/head", get(head_request_handler))
        .route("/heartbeat/:id", get(heartbeat_handler))
        // nesting the authentication related routes
        .nest("/auth", auth::routes())
        // nesting the user related routes
        .nest("/users", users::routes())
        // add a fallback service for handling routes to unknown paths
        .fallback(error_404_handler)
        .with_state(state)
}

#[tracing::instrument(
    level = tracing::Level::TRACE,
    name = "axum",
    skip_all,
    fields(method = request.method().to_string(), uri = request.uri().to_string())
)]
pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    tracing::trace!("received a {} request to {}", request.method(), request.uri());
    next.run(request).await
}

async fn heartbeat_handler(Path(id): Path<u32>) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("heartbeat: received id: {}", id);
    let map = HashMap::from([
        ("service".to_string(), SERVICE_NAME.to_string()),
        ("version".to_string(), SERVICE_VERSION.to_string()),
        ("heartbeat-id".to_string(), id.to_string()),
    ]);
    Ok(Json(map))
}

async fn head_request_handler(method: Method) -> Response {
    // it usually only makes sense to special-case HEAD
    // if computing the body has some relevant cost
    if method == Method::HEAD {
        tracing::debug!("HEAD method found");
        return [("x-some-header", "header from HEAD")].into_response();
    }

    ([("x-some-header", "header from GET")], "body from GET").into_response()
}

async fn error_404_handler(request: Request) -> impl IntoResponse {
    tracing::error!("route not found: {:?}", request);
    ApiError {
        error_message: "Route not found".to_owned(),
        error_type: ApiErrorType::Api,
        status_code: StatusCode::NOT_FOUND,
    }
}
