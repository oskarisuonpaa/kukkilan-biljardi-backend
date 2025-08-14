use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CalendarResponse {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub active: Option<bool>,
}
