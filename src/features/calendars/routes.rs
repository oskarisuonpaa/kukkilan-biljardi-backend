use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    error::AppError,
    features::calendars::{data_transfer_objects::CalendarResponse, model::CalendarRow},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/calendars", get(list))
        .route("/api/calendars/{id}", get(get_by_id))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<CalendarResponse>>, AppError> {
    let rows = state.calendars.list().await?;
    Ok(Json(rows.into_iter().map(to_response).collect()))
}

async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<CalendarResponse>, AppError> {
    match state.calendars.get(id).await? {
        Some(row) => Ok(Json(to_response(row))),
        None => Err(AppError::NotFound("Calendar not found".into())),
    }
}

fn to_response(row: CalendarRow) -> CalendarResponse {
    CalendarResponse {
        id: row.id,
        name: row.name,
    }
}
