use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NoticeRow {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
}
