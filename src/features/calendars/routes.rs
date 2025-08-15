use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    error::AppError,
    features::calendars::{
        data_transfer_objects::{CalendarResponse, CreateCalendarRequest, UpdateCalendarRequest},
        model::CalendarRow,
    },
    response::{Created, NoContent},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/calendars", get(list).post(create))
        .route(
            "/api/calendars/{id}",
            get(get_by_id).put(update).delete(delete),
        )
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<CalendarResponse>>, AppError> {
    let rows = state.calendars.list().await?;
    Ok(Json(rows.into_iter().map(row_to_response).collect()))
}

async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<CalendarResponse>, AppError> {
    let row = state.calendars.get_by_id(id).await?;
    Ok(Json(row_to_response(row)))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateCalendarRequest>,
) -> Result<Created<CalendarResponse>, AppError> {
    let row = state.calendars.create(body).await?;

    Ok(Created {
        location: format!("/api/calendars/{}", row.id),
        body: row_to_response(row),
    })
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(body): Json<UpdateCalendarRequest>,
) -> Result<NoContent, AppError> {
    state.calendars.update(id, body).await?;
    Ok(NoContent)
}

async fn delete(State(state): State<AppState>, Path(id): Path<u64>) -> Result<NoContent, AppError> {
    state.calendars.delete(id).await?;
    Ok(NoContent)
}

fn row_to_response(row: CalendarRow) -> CalendarResponse {
    CalendarResponse {
        id: row.id,
        name: row.name,
    }
}
