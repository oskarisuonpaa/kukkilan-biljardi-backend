use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, put},
};

use super::{
    data_transfer_objects as dto,
    model::{OpeningExceptionRow, OpeningHourRow},
};

use crate::{error::AppError, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/opening-hours", get(list_hours))
        .route(
            "/api/opening-hours/{weekday}",
            put(upsert_hour).delete(delete_hour),
        )
        .route("/api/opening-exceptions", get(list_exceptions))
        .route(
            "/api/opening-exceptions/{date}",
            put(upsert_exception).delete(delete_exception),
        )
}

// ----- Hours -----
async fn list_hours(
    State(app): State<AppState>,
) -> Result<Json<Vec<dto::OpeningHourResponse>>, AppError> {
    let rows: Vec<OpeningHourRow> = app.opening_hours.list().await?;
    Ok(Json(rows.into_iter().map(hour_row_to_resp).collect()))
}

async fn upsert_hour(
    State(app): State<AppState>,
    Path(weekday): Path<u8>,
    Json(body): Json<dto::UpsertOpeningHourRequest>,
) -> Result<StatusCode, AppError> {
    app.opening_hours
        .upsert(weekday, &body.opens_at, &body.closes_at)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_hour(
    State(app): State<AppState>,
    Path(weekday): Path<u8>,
) -> Result<(StatusCode, Json<u64>), AppError> {
    let affected = app.opening_hours.delete_weekday(weekday).await?;
    Ok((StatusCode::OK, Json(affected)))
}

fn hour_row_to_resp(r: OpeningHourRow) -> dto::OpeningHourResponse {
    dto::OpeningHourResponse {
        weekday: r.weekday,
        opens_at: r.opens_at.format("%H:%M:%S").to_string(),
        closes_at: r.closes_at.format("%H:%M:%S").to_string(),
    }
}

// ----- Exceptions -----
async fn list_exceptions(
    State(app): State<AppState>,
    Query(q): Query<dto::ExceptionsQuery>,
) -> Result<Json<Vec<dto::OpeningExceptionResponse>>, AppError> {
    let rows: Vec<OpeningExceptionRow> = app
        .opening_exceptions
        .list(q.from.as_deref(), q.to.as_deref())
        .await?;
    Ok(Json(rows.into_iter().map(exception_row_to_resp).collect()))
}

async fn upsert_exception(
    State(app): State<AppState>,
    Path(date): Path<String>,
    Json(body): Json<dto::UpsertOpeningExceptionRequest>,
) -> Result<StatusCode, AppError> {
    app.opening_exceptions
        .upsert(&date, body.is_closed, &body.opens_at, &body.closes_at)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_exception(
    State(app): State<AppState>,
    Path(date): Path<String>,
) -> Result<(StatusCode, Json<u64>), AppError> {
    let affected = app.opening_exceptions.delete(&date).await?;
    Ok((StatusCode::OK, Json(affected)))
}

fn exception_row_to_resp(r: OpeningExceptionRow) -> dto::OpeningExceptionResponse {
    dto::OpeningExceptionResponse {
        date: r.date.format("%Y-%m-%d").to_string(),
        is_closed: r.is_closed,
        opens_at: r.opens_at.map(|t| t.format("%H:%M:%S").to_string()),
        closes_at: r.closes_at.map(|t| t.format("%H:%M:%S").to_string()),
    }
}
