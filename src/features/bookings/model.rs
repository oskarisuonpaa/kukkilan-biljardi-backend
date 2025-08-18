use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BookingRow {
    pub id: u64,
    pub calendar_id: u64,
    pub name: String,
    pub email: String,
    pub phone: String,
    //pub details: String,
    // pub start: NaiveDateTime,
    // pub end: NaiveDateTime,
    pub created_at: NaiveDateTime,
    //pub updated_at: NaiveDateTime,
}
