use axum::{Json, Router, extract::State, routing::get};

use crate::{
    error::AppError,
    features::calendars::{data_transfer_objects::CalendarResponse, model::CalendarRow},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/calendars", get(list))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<CalendarResponse>>, AppError> {
    let rows = state.calendars.list().await?;
    Ok(Json(rows.into_iter().map(to_response).collect()))
}

fn to_response(row: CalendarRow) -> CalendarResponse {
    CalendarResponse {
        id: row.id,
        name: row.name,
    }
}
