use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MediaRow {
    pub id: u32,
    pub is_background: bool,
    pub kind: String, // 'image' or 'video'
    pub file_url: String,
    pub alt_text: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
