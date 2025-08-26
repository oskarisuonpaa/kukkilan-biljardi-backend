use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContactInfoRow {
    pub id: u32,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub updated_at: NaiveDateTime,
}
