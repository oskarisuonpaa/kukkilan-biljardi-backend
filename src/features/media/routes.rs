use super::{
    data_transfer_objects::{CreateMediaRequest, MediaResponse, UpdateMediaRequest},
    model::MediaRow,
};
use crate::{
    error::AppError,
    response::{Created, NoContent},
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/media", get(list).post(create))
        .route(
            "/api/media/:id",
            get(get_by_id).patch(update).delete(delete_one),
        )
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    is_background: Option<bool>,
}

async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<MediaResponse>>, AppError> {
    let rows = state.media.list(q.is_background).await?;
    Ok(Json(rows.into_iter().map(row_to_response).collect()))
}

async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<MediaResponse>, AppError> {
    let row = state.media.get(id).await?;
    Ok(Json(row_to_response(row)))
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateMediaRequest>,
) -> Result<Created<MediaResponse>, AppError> {
    let new_id = state.media.create(body).await?;
    let row = state.media.get(new_id).await?;
    Ok(Created {
        location: format!("/api/media/{new_id}"),
        body: row_to_response(row),
    })
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(body): Json<UpdateMediaRequest>,
) -> Result<Json<MediaResponse>, AppError> {
    state.media.update(id, body).await?;
    let row = state.media.get(id).await?;
    Ok(Json(row_to_response(row)))
}

async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<NoContent, AppError> {
    state.media.delete(id).await?;
    Ok(NoContent)
}

fn row_to_response(row: MediaRow) -> MediaResponse {
    MediaResponse {
        id: row.id,
        is_background: row.is_background,
        kind: row.kind,
        file_url: row.file_url,
        alt_text: row.alt_text,
        width: row.width,
        height: row.height,
    }
}
