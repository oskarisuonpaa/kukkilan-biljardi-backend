use super::data_transfer_objects::BookingResponse;
use crate::{error::AppError, features::bookings::model::BookingRow, state::AppState};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/calendar/{calendar_id}/bookings", get(list))
}

async fn list(
    State(state): State<AppState>,
    Path(calendar_id): Path<u64>,
) -> Result<Json<Vec<BookingResponse>>, AppError> {
    let rows = state.bookings.list(calendar_id).await?;
    Ok(Json(rows.into_iter().map(row_to_response).collect()))
}

fn row_to_response(row: BookingRow) -> BookingResponse {
    BookingResponse {
        id: row.id,
        calendar_id: row.calendar_id,
        name: row.name,
        email: row.email,
        phone: row.phone,
    }
}
