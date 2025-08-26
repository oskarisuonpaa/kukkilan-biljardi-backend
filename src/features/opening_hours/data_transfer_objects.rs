use serde::{Deserialize, Serialize};

// ===== Opening Hours =====
#[derive(Debug, Serialize)]
pub struct OpeningHourResponse {
    pub weekday: u8,       // 1=Mon … 7=Sun
    pub opens_at: String,  // "HH:MM:SS"
    pub closes_at: String, // "HH:MM:SS"
}

#[derive(Debug, Deserialize)]
pub struct UpsertOpeningHourRequest {
    pub opens_at: String,  // required
    pub closes_at: String, // required
}

// ===== Opening Exceptions =====
#[derive(Debug, Serialize)]
pub struct OpeningExceptionResponse {
    pub date: String, // "YYYY-MM-DD"
    pub is_closed: bool,
    pub opens_at: Option<String>,
    pub closes_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertOpeningExceptionRequest {
    pub is_closed: bool,
    pub opens_at: Option<String>,
    pub closes_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExceptionsQuery {
    pub from: Option<String>, // YYYY-MM-DD
    pub to: Option<String>,   // YYYY-MM-DD
}
