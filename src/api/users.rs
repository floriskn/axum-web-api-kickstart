use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{ delete, get, post, put },
    Router,
};
use sqlx::types::Uuid;

use crate::{
    application::{
        api_error::{ ApiError, ApiErrorType },
        api_json::Json,
        api_path::Path,
        repository::user_repo,
        security::jwt_claims::{ AccessClaims, ClaimsMethods },
        state::SharedState,
    },
    domain::models::user::User,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_users_handler))
        .route("/", post(add_user_handler))
        .route("/:id", get(get_user_handler))
        .route("/:id", put(update_user_handler))
        .route("/:id", delete(delete_user_handler))
}

async fn list_users_handler(
    access_claims: AccessClaims,
    State(state): State<SharedState>
) -> Result<Json<Vec<User>>, ApiError> {
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    match user_repo::all_users(&state).await {
        Some(users) => Ok(Json(users)),
        None => Err(StatusCode::NOT_FOUND.into()),
    }
}

async fn add_user_handler(
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(user): Json<User>
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    match user_repo::add_user(user, &state).await {
        Some(user) => Ok((StatusCode::CREATED, Json(user))),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
    }
}

async fn get_user_handler(
    access_claims: AccessClaims,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>
) -> Result<Json<User>, ApiError> {
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    match user_repo::get_user(id, &state).await {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND.into()),
    }
}

async fn update_user_handler(
    access_claims: AccessClaims,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(user): Json<User>
) -> Result<Json<User>, ApiError> {
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    match user_repo::update_user(id, user, &state).await {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND.into()),
    }
}

async fn delete_user_handler(
    access_claims: AccessClaims,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    match user_repo::delete_user(id, &state).await {
        Some(true) => Ok(StatusCode::OK),
        Some(false) =>
            Err(ApiError {
                status_code: StatusCode::NOT_FOUND,
                error_type: ApiErrorType::Api,
                error_message: format!("User not found for deletion: {}", id),
            }),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
    }
}
