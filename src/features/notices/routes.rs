use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, put},
};

use crate::{
    error::AppError,
    features::notices::{
        data_transfer_objects::{CreateNoticeRequest, NoticeResponse, UpdateNoticeRequest},
        model::NoticeRow,
    },
    response::{Created, NoContent},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/notices", get(list_notices).post(create_notice))
        .route(
            "/api/notices/{id}",
            put(update_notice).delete(delete_notice),
        )
}

async fn list_notices(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<NoticeResponse>>, AppError> {
    let notice_rows = app_state.notices.list().await?;
    let responses = notice_rows
        .into_iter()
        .map(convert_row_to_response)
        .collect();
    Ok(Json(responses))
}

async fn create_notice(
    State(app_state): State<AppState>,
    Json(request_body): Json<CreateNoticeRequest>,
) -> Result<Created<NoticeResponse>, AppError> {
    let notice_row = app_state.notices.create(request_body).await?;
    Ok(Created {
        location: format!("/api/notices/{}", notice_row.id),
        body: convert_row_to_response(notice_row),
    })
}

async fn update_notice(
    State(app_state): State<AppState>,
    Path(notice_id): Path<u32>,
    Json(request_body): Json<UpdateNoticeRequest>,
) -> Result<(StatusCode, Json<NoticeRow>), AppError> {
    let updated_row = app_state.notices.update(notice_id, request_body).await?;
    Ok((StatusCode::OK, Json(updated_row)))
}

async fn delete_notice(
    State(app_state): State<AppState>,
    Path(notice_id): Path<u32>,
) -> Result<NoContent, AppError> {
    app_state.notices.delete(notice_id).await?;
    Ok(NoContent)
}

fn convert_row_to_response(notice_row: NoticeRow) -> NoticeResponse {
    NoticeResponse {
        id: notice_row.id,
        title: notice_row.title,
        content: notice_row.content,
        active: notice_row.active,
    }
}
