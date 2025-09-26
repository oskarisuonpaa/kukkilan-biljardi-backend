use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CalendarRow {
    pub id: u32,
    pub name: String,
    pub thumbnail_id: Option<u32>,
    pub active: bool,
    pub hourly_price_cents: Option<u32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
