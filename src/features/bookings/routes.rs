use super::data_transfer_objects::{BookingResponse, DailyOverviewResponse};
use crate::{
    error::AppError,
    features::{
        bookings::{data_transfer_objects::CreateBookingRequest, model::BookingRow},
        email::templates::BookingConfirmationData,
    },
    response::{Created, NoContent},
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    http::header::AUTHORIZATION,
    routing::{get, post},
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
struct DailyOverviewQuery {
    date: String,
}

// Public routes that don't require authentication
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/api/calendar/{calendar_id}/bookings", get(list))
        .route("/api/bookings", post(create))
        .route("/api/bookings/{id}", axum::routing::delete(delete))
}

// Admin routes that require authentication
pub fn admin_routes() -> Router<AppState> {
    Router::new().route("/api/admin/bookings/daily-overview", get(daily_overview))
}

// Backward compatibility - combine both routes
pub fn routes() -> Router<AppState> {
    Router::new().merge(public_routes()).merge(admin_routes())
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
    let row = state.bookings.create(body.clone()).await?;

    // Send email confirmation (async, don't fail booking if email fails)
    if !body.email.is_empty() {
        let state_clone = state.clone();
        let row_clone = row.clone();
        let booking_task = tokio::spawn(async move {
            // Get calendar details for email
            if let Ok(calendar) = state_clone.calendars.get(row_clone.calendar_id).await {
                let table_type = if calendar.name.to_lowercase().contains("snooker") {
                    "Snooker"
                } else {
                    "Pool"
                }.to_string();

                let duration_hours = (row_clone.ends_at_utc - row_clone.starts_at_utc).num_hours();
                let hourly_price = calendar.hourly_price_cents.unwrap_or(0);
                let total_price = (hourly_price as i64 * duration_hours) as i32;
                
                let booking_data = BookingConfirmationData {
                    customer_name: row_clone.customer_name.clone(),
                    calendar_name: calendar.name.clone(),
                    table_type,
                    booking_date: row_clone.starts_at_utc.date().format("%d.%m.%Y").to_string(),
                    start_time: row_clone.starts_at_utc.time().format("%H:%M").to_string(),
                    end_time: row_clone.ends_at_utc.time().format("%H:%M").to_string(),
                    duration_hours,
                    total_price,
                    booking_id: row_clone.id,
                };

                if let Err(e) = state_clone.email.send_booking_confirmation(
                    &row_clone.customer_email,
                    &row_clone.customer_name,
                    booking_data
                ).await {
                    eprintln!("Failed to send booking confirmation email to {}: {}", row_clone.customer_email, e);
                }
            }
        });

        // Don't await the email task - let it run in background
        let _ = booking_task;
    }

    Ok(Created {
        location: format!("/api/bookings/{}", row.id),
        body: row_to_response(row),
    })
}

async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> Result<NoContent, AppError> {
    state.bookings.delete(id).await?;
    Ok(NoContent)
}

async fn daily_overview(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<DailyOverviewQuery>,
) -> Result<Json<DailyOverviewResponse>, AppError> {
    // Check authorization
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return Err(AppError::Unauthorized(
                "Missing or invalid authorization header".to_string(),
            ));
        }
    };

    // Verify token
    state.auth.verify_token(token)?;

    // Validate the date format
    params
        .date
        .parse::<NaiveDate>()
        .map_err(|_| AppError::BadRequest("Invalid date format"))?;

    let overview = state.bookings.daily_overview(&params.date).await?;
    Ok(Json(overview))
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
