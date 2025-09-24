use super::{
    data_transfer_objects::{CalendarResponse, CreateCalendarRequest, UpdateCalendarRequest},
    model::CalendarRow,
};
use crate::{
    error::AppError,
    features::media::data_transfer_objects::MediaResponse,
    response::{Created, NoContent},
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/calendars", get(list).post(create))
        .route(
            "/api/calendars/:id",
            get(get_by_id).patch(update).delete(delete_one),
        )
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<CalendarResponse>>, AppError> {
    let rows = state.calendars.list().await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let thumb = match row.thumbnail_id {
            Some(mid) => Some(media_to_response(state.media.get(mid).await?)),
            None => None,
        };
        out.push(row_to_response(row, thumb));
    }
    Ok(Json(out))
}

async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<CalendarResponse>, AppError> {
    let row = state.calendars.get(id).await?;
    let thumb = match row.thumbnail_id {
        Some(mid) => Some(media_to_response(state.media.get(mid).await?)),
        None => None,
    };
    Ok(Json(row_to_response(row, thumb)))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateCalendarRequest>,
) -> Result<Created<CalendarResponse>, AppError> {
    let new_id = state.calendars.create(body).await?;
    let row = state.calendars.get(new_id).await?;
    let thumb = match row.thumbnail_id {
        Some(mid) => Some(media_to_response(state.media.get(mid).await?)),
        None => None,
    };
    Ok(Created {
        location: format!("/api/calendars/{new_id}"),
        body: row_to_response(row, thumb),
    })
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(body): Json<UpdateCalendarRequest>,
) -> Result<Json<CalendarResponse>, AppError> {
    state.calendars.update(id, body).await?;
    let row = state.calendars.get(id).await?;
    let thumb = match row.thumbnail_id {
        Some(mid) => Some(media_to_response(state.media.get(mid).await?)),
        None => None,
    };
    Ok(Json(row_to_response(row, thumb)))
}

async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<NoContent, AppError> {
    state.calendars.delete(id).await?;
    Ok(NoContent)
}

fn row_to_response(row: CalendarRow, thumbnail: Option<MediaResponse>) -> CalendarResponse {
    CalendarResponse {
        id: row.id,
        name: row.name,
        active: row.active,
        thumbnail,
    }
}

fn media_to_response(m: crate::features::media::model::MediaRow) -> MediaResponse {
    MediaResponse {
        id: m.id,
        is_background: m.is_background,
        kind: m.kind,
        file_url: m.file_url,
        alt_text: m.alt_text,
        width: m.width,
        height: m.height,
    }
}
