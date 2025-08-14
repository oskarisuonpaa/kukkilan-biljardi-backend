use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    error::AppError,
    features::calendars::{
        data_transfer_objects::{CalendarResponse, CreateCalendarRequest},
        model::CalendarRow,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/calendars", get(list).post(create))
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
    let row = state.calendars.get_by_id(id).await?;
    Ok(Json(to_response(row)))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateCalendarRequest>,
) -> Result<Json<CalendarResponse>, AppError> {
    let row = state.calendars.create(body).await?;
    Ok(Json(to_response(row)))
}

// async fn update(
//     State(state): State<AppState>,
//     Path(id): Path<u64>,
//     Json(body): Json<UpdateCalendarRequest>,
// ) -> Result<Json<CalendarResponse>, AppError> {
//      let row = state.calendars.update(body).await?;
//      Ok(Json(to_response(row)))
// }

fn to_response(row: CalendarRow) -> CalendarResponse {
    CalendarResponse {
        id: row.id,
        name: row.name,
    }
}
