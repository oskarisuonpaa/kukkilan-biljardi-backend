use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CalendarResponse {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCalendarRequest {
    pub name: String,
    pub active: Option<bool>,
}
