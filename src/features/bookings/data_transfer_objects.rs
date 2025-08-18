use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: u32,
    pub calendar_id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
