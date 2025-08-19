use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: u32,
    pub calendar_id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub notes: Option<String>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub calendar_id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub notes: Option<String>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
