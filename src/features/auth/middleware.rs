use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use crate::{
    features::auth::data_transfer_objects::TokenClaims,
    state::AppState,
    error::AppError,
};

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return Err(AppError::Unauthorized("Missing or invalid authorization header".to_string()));
        }
    };

    // Verify token
    match state.auth.verify_token(token) {
        Ok(claims) => {
            // Add claims to request extensions for use in handlers
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(e) => Err(e),
    }
}