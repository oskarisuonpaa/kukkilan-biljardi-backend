use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime};
use sqlx::{MySql, Pool};

use crate::features::opening::model::{OpeningExceptionRow, OpeningHourRow};

// ---------- Traits ----------
#[async_trait]
pub trait OpeningHoursRepository: Send + Sync {
    async fn list(&self) -> sqlx::Result<Vec<OpeningHourRow>>;
    async fn upsert(
        &self,
        weekday: u8,
        opens_at: NaiveTime,
        closes_at: NaiveTime,
    ) -> sqlx::Result<()>;
    async fn delete_weekday(&self, weekday: u8) -> sqlx::Result<u64>;
}

#[async_trait]
pub trait OpeningExceptionsRepository: Send + Sync {
    async fn list(
        &self,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> sqlx::Result<Vec<OpeningExceptionRow>>;
    async fn upsert(
        &self,
        date: NaiveDate,
        is_closed: bool,
        opens_at: Option<NaiveTime>,
        closes_at: Option<NaiveTime>,
    ) -> sqlx::Result<()>;
    async fn delete(&self, date: NaiveDate) -> sqlx::Result<u64>;
}

pub type DynOpeningHoursRepo = std::sync::Arc<dyn OpeningHoursRepository>;
pub type DynOpeningExceptionsRepo = std::sync::Arc<dyn OpeningExceptionsRepository>;

// ---------- MySQL impls ----------
#[derive(Clone)]
pub struct MySqlOpeningHoursRepository {
    pool: Pool<MySql>,
}

impl MySqlOpeningHoursRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OpeningHoursRepository for MySqlOpeningHoursRepository {
    async fn list(&self) -> sqlx::Result<Vec<OpeningHourRow>> {
        sqlx::query_as!(
            OpeningHourRow,
            r#"SELECT id as `id: u32`, weekday as `weekday: u8`, opens_at, closes_at FROM opening_hours ORDER BY weekday"#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn upsert(
        &self,
        weekday: u8,
        opens_at: NaiveTime,
        closes_at: NaiveTime,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO opening_hours (weekday, opens_at, closes_at)
                VALUES (?, ?, ?)
                ON DUPLICATE KEY UPDATE
                    opens_at = VALUES(opens_at),
                    closes_at = VALUES(closes_at)
            "#,
            weekday,
            opens_at,
            closes_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_weekday(&self, weekday: u8) -> sqlx::Result<u64> {
        let res = sqlx::query!("DELETE FROM opening_hours WHERE weekday = ?", weekday)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}

#[derive(Clone)]
pub struct MySqlOpeningExceptionsRepository {
    pool: Pool<MySql>,
}

impl MySqlOpeningExceptionsRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OpeningExceptionsRepository for MySqlOpeningExceptionsRepository {
    async fn list(
        &self,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> sqlx::Result<Vec<OpeningExceptionRow>> {
        match (from, to) {
            (None, None) => sqlx::query_as!(
                OpeningExceptionRow,
                r#"SELECT id as `id: u32`, date, is_closed, opens_at, closes_at FROM opening_exceptions ORDER BY date"#
            )
            .fetch_all(&self.pool)
            .await,
            (Some(f), None) => sqlx::query_as!(
                OpeningExceptionRow,
                r#"SELECT id as `id: u32`, date, is_closed, opens_at, closes_at FROM opening_exceptions WHERE date >= ? ORDER BY date"#,
                f
            )
            .fetch_all(&self.pool)
            .await,
            (None, Some(t)) => sqlx::query_as!(
                OpeningExceptionRow,
                r#"SELECT id as `id: u32`, date, is_closed, opens_at, closes_at FROM opening_exceptions WHERE date <= ? ORDER BY date"#,
                t
            )
            .fetch_all(&self.pool)
            .await,
            (Some(f), Some(t)) => sqlx::query_as!(
                OpeningExceptionRow,
                r#"SELECT id as `id: u32`, date, is_closed, opens_at, closes_at FROM opening_exceptions WHERE date BETWEEN ? AND ? ORDER BY date"#,
                f,
                t
            )
            .fetch_all(&self.pool)
            .await,
        }
    }

    async fn upsert(
        &self,
        date: NaiveDate,
        is_closed: bool,
        opens_at: Option<NaiveTime>,
        closes_at: Option<NaiveTime>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO opening_exceptions (date, is_closed, opens_at, closes_at)
                VALUES (?, ?, ?, ?)
                ON DUPLICATE KEY UPDATE
                    is_closed = VALUES(is_closed),
                    opens_at = VALUES(opens_at),
                    closes_at = VALUES(closes_at)
            "#,
            date,
            is_closed,
            opens_at,
            closes_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, date: NaiveDate) -> sqlx::Result<u64> {
        let res = sqlx::query!("DELETE FROM opening_exceptions WHERE date = ?", date)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}
