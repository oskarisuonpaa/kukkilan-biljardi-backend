use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OpeningHourRow {
    pub id: u32,
    pub weekday: u8,
    pub opens_at: NaiveTime,
    pub closes_at: NaiveTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OpeningExceptionRow {
    pub id: u32,
    pub date: NaiveDate,
    pub is_closed: bool,
    pub opens_at: Option<NaiveTime>,
    pub closes_at: Option<NaiveTime>,
}
