use crate::features::bookings::data_transfer_objects::CreateBookingRequest;

use super::model::BookingRow;
use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[async_trait]
pub trait BookingsRepository: Send + Sync {
    async fn list(&self, calendar_id: u32) -> sqlx::Result<Vec<BookingRow>>;
    async fn get(&self, id: u32) -> sqlx::Result<Option<BookingRow>>;
    async fn insert(&self, data: CreateBookingRequest) -> sqlx::Result<u32>;
    async fn delete(&self, id: u32) -> sqlx::Result<bool>;
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
    async fn list(&self, calendar_id: u32) -> sqlx::Result<Vec<BookingRow>> {
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
                starts_at_utc: row.starts_at_utc.clone(),
                ends_at_utc: row.ends_at_utc.clone(),
                customer_name: row.customer_name,
                customer_email: row.customer_email,
                customer_phone: row.customer_phone,
                customer_notes: row.customer_notes,
                created_at: row.created_at.clone(),
                updated_at: row.updated_at.clone(),
            })
            .collect())
    }

    async fn get(&self, id: u32) -> sqlx::Result<Option<BookingRow>> {
        let row = sqlx::query!(r#"SELECT * FROM bookings WHERE id = ?"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| BookingRow {
            id: row.id,
            calendar_id: row.calendar_id,
            starts_at_utc: row.starts_at_utc.clone(),
            ends_at_utc: row.ends_at_utc.clone(),
            customer_name: row.customer_name,
            customer_email: row.customer_email,
            customer_phone: row.customer_phone,
            customer_notes: row.customer_notes,
            created_at: row.created_at.clone(),
            updated_at: row.updated_at.clone(),
        }))
    }

    async fn insert(&self, data: CreateBookingRequest) -> sqlx::Result<u32> {
        let result = sqlx::query!(
            r#"INSERT INTO bookings (calendar_id, starts_at_utc, ends_at_utc, customer_name, customer_email, customer_phone, customer_notes) VALUES (?,?,?,?,?,?,?)"#,
            data.calendar_id,
            data.start,
            data.end,
            data.name,
            data.email,
            data.phone,
            data.notes
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    async fn delete(&self, id: u32) -> sqlx::Result<bool> {
        let result = sqlx::query!(r#"DELETE FROM bookings WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
