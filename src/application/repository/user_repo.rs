use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    application::{ app_const::USER_ROLE_GUEST, security::password, state::SharedState },
    domain::models::user::User,
};

pub async fn all_users(state: &SharedState) -> Option<Vec<User>> {
    match query_as::<_, User>("SELECT * FROM users").fetch_all(&state.pgpool).await {
        Ok(users) => Some(users),
        Err(e) => {
            tracing::error!("{}", e);
            None
        }
    }
}

pub async fn add_user(user: User, state: &SharedState) -> Option<User> {
    let time_now = Utc::now().naive_utc();
    tracing::trace!("user: {:#?}", user);

    let password = password::hash_password(user.password.as_bytes());

    let query_add = sqlx
        ::query_as::<_, User>(
            r#"INSERT INTO users (id,
         username,
         email,
         password,
         active,
         roles,
         created_at,
         updated_at)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
         RETURNING users.*"#
        )
        .bind(user.id)
        .bind(user.username)
        .bind(user.email)
        .bind(password)
        .bind(true)
        .bind(USER_ROLE_GUEST)
        .bind(time_now)
        .bind(time_now)
        .fetch_one(&state.pgpool).await;

    match query_add {
        Ok(user) => Some(user),
        Err(e) => {
            tracing::error!("{}", e);
            None
        }
    }
}

pub async fn get_user(id: Uuid, state: &SharedState) -> Option<User> {
    let query_get = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pgpool).await;

    match query_get {
        Ok(user) => Some(user),
        Err(e) => {
            tracing::error!("{}", e);
            None
        }
    }
}

pub async fn get_user_by_username(username: &str, state: &SharedState) -> Option<User> {
    let query_get = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(&state.pgpool).await;

    match query_get {
        Ok(user) => Some(user),
        Err(e) => {
            tracing::error!("{}", e);
            None
        }
    }
}

pub async fn update_user(id: Uuid, user: User, state: &SharedState) -> Option<User> {
    tracing::trace!("user: {:#?}", user);
    let time_now = Utc::now().naive_utc();
    let query_update = sqlx
        ::query_as::<_, User>(
            r#"UPDATE users
         SET id = $1,
         username = $2,
         email = $3,
         password = $4,
         active = $6,
         roles = $7,
         updated_at = $8
         WHERE id = $9
         RETURNING users.*"#
        )
        .bind(user.id)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password)
        .bind(user.active)
        .bind(user.roles)
        .bind(time_now)
        .bind(id)
        .fetch_one(&state.pgpool).await;

    match query_update {
        Ok(user) => Some(user),
        Err(e) => {
            tracing::error!("{}", e);
            None
        }
    }
}

pub async fn delete_user(id: Uuid, state: &SharedState) -> Option<bool> {
    let query_delete = sqlx
        ::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.pgpool).await;

    match query_delete {
        Ok(row) => Some(row.rows_affected() == 1),
        Err(e) => {
            tracing::error!("{}", e);
            None
        }
    }
}
