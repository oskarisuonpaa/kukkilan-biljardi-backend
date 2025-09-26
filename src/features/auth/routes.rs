use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use crate::{
    features::auth::{
        data_transfer_objects::{LoginRequest, LoginResponse},
        model::AdminSession,
    },
    state::AppState,
    error::AppError,
    response::NoContent,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(current_user))
}

async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = state.auth.authenticate(request).await?;
    Ok(Json(response))
}

async fn logout() -> Result<NoContent, AppError> {
    // With JWT, logout is handled client-side by removing the token
    Ok(NoContent)
}

async fn current_user() -> Result<Json<Option<AdminSession>>, AppError> {
    // This will be updated when we implement the auth middleware
    // For now, return None
    Ok(Json(None))
}