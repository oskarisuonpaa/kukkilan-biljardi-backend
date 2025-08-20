use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CalendarResponse {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateCalendarRequest {
    pub name: String,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCalendarRequest {
    pub name: Option<String>,
    pub active: Option<bool>,
}
