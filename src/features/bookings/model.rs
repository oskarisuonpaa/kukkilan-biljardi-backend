use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BookingRow {
    pub id: u32,
    pub calendar_id: u32,
    pub starts_at_utc: NaiveDateTime,
    pub ends_at_utc: NaiveDateTime,
    pub customer_name: String,
    pub customer_email: String,
    pub customer_phone: String,
    pub customer_notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
