use chrono::{NaiveDate, NaiveTime};

use super::repository::{DynOpeningExceptionsRepo, DynOpeningHoursRepo};
use crate::error::AppError;
use crate::features::opening_hours::model::{OpeningExceptionRow, OpeningHourRow};

#[derive(Clone)]
pub struct OpeningHoursService {
    repo: DynOpeningHoursRepo,
}
impl OpeningHoursService {
    pub fn new(repo: DynOpeningHoursRepo) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<OpeningHourRow>, AppError> {
        Ok(self.repo.list().await?)
    }

    pub async fn upsert(
        &self,
        weekday: u8,
        opens_at_s: &str,
        closes_at_s: &str,
    ) -> Result<(), AppError> {
        if !(1..=7).contains(&weekday) {
            return Err(AppError::BadRequest("weekday must be 1..=7"));
        }
        let opens = parse_time(opens_at_s)?;
        let closes = parse_time(closes_at_s)?;
        if opens >= closes {
            return Err(AppError::BadRequest("opens_at must be before closes_at"));
        }
        Ok(self.repo.upsert(weekday, opens, closes).await?)
    }

    pub async fn delete_weekday(&self, weekday: u8) -> Result<u64, AppError> {
        if !(1..=7).contains(&weekday) {
            return Err(AppError::BadRequest("weekday must be 1..=7"));
        }
        Ok(self.repo.delete_weekday(weekday).await?)
    }
}

#[derive(Clone)]
pub struct OpeningExceptionsService {
    repo: DynOpeningExceptionsRepo,
}
impl OpeningExceptionsService {
    pub fn new(repo: DynOpeningExceptionsRepo) -> Self {
        Self { repo }
    }

    pub async fn list(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<OpeningExceptionRow>, AppError> {
        let from_d = match from {
            Some(s) => Some(parse_date(s)?),
            None => None,
        };
        let to_d = match to {
            Some(s) => Some(parse_date(s)?),
            None => None,
        };
        Ok(self.repo.list(from_d, to_d).await?)
    }

    pub async fn upsert(
        &self,
        date_s: &str,
        is_closed: bool,
        opens_at: &Option<String>,
        closes_at: &Option<String>,
    ) -> Result<(), AppError> {
        let date = parse_date(date_s)?;
        let (o, c) = if is_closed {
            (None, None)
        } else {
            let o = opens_at.as_deref().ok_or(AppError::BadRequest(
                "opens_at required when is_closed=false",
            ))?;
            let c = closes_at.as_deref().ok_or(AppError::BadRequest(
                "closes_at required when is_closed=false",
            ))?;
            let o_t = parse_time(o)?;
            let c_t = parse_time(c)?;
            if o_t >= c_t {
                return Err(AppError::BadRequest("opens_at must be before closes_at"));
            }
            (Some(o_t), Some(c_t))
        };
        Ok(self.repo.upsert(date, is_closed, o, c).await?)
    }

    pub async fn delete(&self, date_s: &str) -> Result<u64, AppError> {
        Ok(self.repo.delete(parse_date(date_s)?).await?)
    }
}

// ---- helpers ----
fn parse_time(s: &str) -> Result<NaiveTime, AppError> {
    NaiveTime::parse_from_str(s, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M"))
        .map_err(|_| AppError::BadRequest("time must be HH:MM[:SS]"))
}
fn parse_date(s: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("date must be YYYY-MM-DD"))
}
