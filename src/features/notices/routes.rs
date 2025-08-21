use axum::{Json, Router, extract::State, routing::get};

use crate::{
    error::AppError,
    features::notices::{data_transfer_objects::NoticeResponse, model::NoticeRow},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/notices", get(list)) //.post(create)
    // .route("/api/notices/{id}", put(update).delete(delete))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<NoticeResponse>>, AppError> {
    let rows = state.notices.list().await?;
    Ok(Json(rows.into_iter().map(row_to_response).collect()))
}

// async fn create(
//     State(state): State<AppState>,
//     Json(body): Json<CreateNoticeRequest>,
// ) -> Result<Created<NoticeResponse>, AppError> {
//     let row = state.notices.create(body).await?;

//     Ok(Created {
//         location: format!("/api/notices/{}", row.id),
//         body: row_to_response(row),
//     })
// }

// async fn update(
//     State(state): State<AppState>,
//     Path(id): Path<u32>,
//     Json(body): Json<UpdateNoticeRequest>,
// ) -> Result<(StatusCode, Json<NoticeRow>), AppError> {
//     let row = state.notices.update(id, body).await?;
//     Ok((StatusCode::OK, Json(row)))
// }

// async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> Result<NoContent, AppError> {
//     state.notices.delete(id).await?;
//     Ok(NoContent)
// }

fn row_to_response(row: NoticeRow) -> NoticeResponse {
    NoticeResponse {
        id: row.id,
        title: row.title,
        content: row.content,
        active: row.active,
    }
}
