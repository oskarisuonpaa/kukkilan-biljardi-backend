use super::model::BookingRow;
use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[async_trait]
pub trait BookingsRepository: Send + Sync {
    async fn list(&self, calendar_id: u64) -> sqlx::Result<Vec<BookingRow>>;
}

pub type DynamicBookingsRepository = std::sync::Arc<dyn BookingsRepository>;

#[derive(Clone)]
pub struct MySqlBookingsRepository {
    pool: Pool<MySql>,
}
impl MySqlBookingsRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookingsRepository for MySqlBookingsRepository {
    async fn list(&self, calendar_id: u64) -> sqlx::Result<Vec<BookingRow>> {
        let rows = sqlx::query!(
            r#"SELECT * FROM bookings WHERE calendar_id = ?"#,
            calendar_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| BookingRow {
                id: row.id,
                calendar_id: row.calendar_id,
                name: row.name,
                email: row.email,
                phone: row.phone,
                //details: row.details,
                //start: row.start.naive_utc(),
                //end: row.end.naive_utc(),
                created_at: row.created_at.naive_utc(),
                //updated_at: row.updated_at.naive_utc(),
            })
            .collect())
    }
}
