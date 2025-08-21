use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notice {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}
