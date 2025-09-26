use serde::{Deserialize, Serialize};

use crate::features::media::data_transfer_objects::MediaResponse;

#[derive(Debug, Serialize)]
pub struct CalendarResponse {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub hourly_price_cents: Option<u32>,
    pub thumbnail: Option<MediaResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCalendarRequest {
    pub name: String,
    pub active: Option<bool>,
    pub thumbnail_id: Option<u32>,
    pub hourly_price_cents: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCalendarRequest {
    pub name: Option<String>,
    pub active: Option<bool>,
    pub thumbnail_id: Option<u32>,
    pub hourly_price_cents: Option<u32>,
}
