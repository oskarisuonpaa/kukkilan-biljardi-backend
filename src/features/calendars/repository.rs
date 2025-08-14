use super::model::CalendarRow;
use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[async_trait]
pub trait CalendarsRepository: Send + Sync {
    async fn list(&self) -> sqlx::Result<Vec<CalendarRow>>;
    // async fn insert(&self, name: &str, active: Option<bool>) -> sqlx::Result<u64>;
    // async fn update(&self, id: u64, name: Option<&str>, active: Option<bool>) -> sqlx::Result<bool>;
    // async fn delete(&self, id: u64) -> sqlx::Result<bool>;
}

pub type DynamicCalendarsRepository = std::sync::Arc<dyn CalendarsRepository>;

#[derive(Clone)]
pub struct MySqlCalendarsRepository {
    pool: Pool<MySql>,
}
impl MySqlCalendarsRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalendarsRepository for MySqlCalendarsRepository {
    async fn list(&self) -> sqlx::Result<Vec<CalendarRow>> {
        let rows = sqlx::query!(
            r#"Select *
            FROM calendars"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| CalendarRow {
                id: row.id,
                name: row.name,
                active: row.active != 0,
                created_at: row.created_at.naive_utc(),
                updated_at: row.updated_at.naive_utc(),
            })
            .collect())
    }
}
