use axum::{extract::State, Json, Router, routing};
use models::vertices::user::{PasswordUser, User};

use super::AppError;
use crate::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::post(add_user))
        .route("/trust", routing::post(trust_users))
        .with_state(state)
}

#[derive(Debug, serde::Deserialize)]
struct AddUserRequest {
    email: String,
    handle: String,
    password: String,
    name: String,
}
async fn add_user(
    State(state): State<AppState>,
    Json(payload): Json<AddUserRequest>,
) -> Result<Json<User>, AppError> {
    let user = state.user_service.add_user(PasswordUser::new(
        &payload.email,
        &payload.handle,
        &payload.password,
        &payload.name,
    ))?;
    Ok(Json(user))
}

#[derive(Debug, serde::Deserialize)]
struct TrustUsersRequest {
    source_id: i64,
    target_id: i64,
}
async fn trust_users(
    State(state): State<AppState>,
    Json(payload): Json<TrustUsersRequest>,
) -> Result<Json<()>, AppError> {
    state
        .user_service
        .trust_users(payload.source_id, payload.target_id)?;
    Ok(Json(()))
}
