use super::data_transfer_objects::BookingResponse;
use crate::{
    error::AppError,
    features::bookings::{data_transfer_objects::CreateBookingRequest, model::BookingRow},
    response::{Created, NoContent},
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/calendar/{calendar_id}/bookings", get(list))
        .route("/api/bookings", post(create))
        .route("/api/bookings/{id}", axum::routing::delete(delete))
}

async fn list(
    State(state): State<AppState>,
    Path(calendar_id): Path<u32>,
) -> Result<Json<Vec<BookingResponse>>, AppError> {
    let rows = state.bookings.list(calendar_id).await?;
    Ok(Json(rows.into_iter().map(row_to_response).collect()))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateBookingRequest>,
) -> Result<Created<BookingResponse>, AppError> {
    let row = state.bookings.create(body).await?;

    Ok(Created {
        location: format!("/api/bookings/{}", row.id),
        body: row_to_response(row),
    })
}

async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> Result<NoContent, AppError> {
    state.bookings.delete(id).await?;
    Ok(NoContent)
}

fn row_to_response(row: BookingRow) -> BookingResponse {
    BookingResponse {
        id: row.id,
        calendar_id: row.calendar_id,
        name: row.customer_name,
        email: row.customer_email,
        phone: row.customer_phone,
        notes: row.customer_notes,
        start: DateTime::<Utc>::from_naive_utc_and_offset(row.starts_at_utc, Utc),
        end: DateTime::<Utc>::from_naive_utc_and_offset(row.ends_at_utc, Utc),
    }
}
