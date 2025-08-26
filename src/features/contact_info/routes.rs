use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, put},
};

use crate::{
    error::AppError,
    features::contact_info::{
        data_transfer_objects::{ContactInfoResponse, UpdateContactInfoRequest},
        model::ContactInfoRow,
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/api/contact-info",
        get(get_contact_info).put(update_contact_info),
    )
}

async fn get_contact_info(
    State(app_state): State<AppState>,
) -> Result<Json<ContactInfoResponse>, AppError> {
    let row = app_state.contact_info.get().await?;
    Ok(Json(convert_row_to_response(row)))
}

async fn update_contact_info(
    State(app_state): State<AppState>,
    Json(request_body): Json<UpdateContactInfoRequest>,
) -> Result<(StatusCode, Json<ContactInfoRow>), AppError> {
    let updated = app_state.contact_info.update(request_body).await?;
    Ok((StatusCode::OK, Json(updated)))
}

fn convert_row_to_response(row: ContactInfoRow) -> ContactInfoResponse {
    ContactInfoResponse {
        address: row.address,
        phone: row.phone,
        email: row.email,
        updated_at: row.updated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    }
}
