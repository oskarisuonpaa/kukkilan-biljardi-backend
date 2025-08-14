use axum::{Router, response::IntoResponse, routing::get};

use crate::state::AppState;

async fn hello_handler() -> impl IntoResponse {
    "Hello"
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/calendars", get(hello_handler))
}
