use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: u64,
    pub calendar_id: u64,
    pub name: String,
    pub email: String,
    pub phone: String,
    // pub start: NaiveDateTime,
    // pub end: NaiveDateTime,
}
