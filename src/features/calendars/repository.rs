use super::model::CalendarRow;
use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[async_trait]
pub trait CalendarsRepository: Send + Sync {
    async fn list(&self) -> sqlx::Result<Vec<CalendarRow>>;
    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<CalendarRow>>;
    async fn insert(
        &self,
        name: &str,
        active: bool,
        thumbnail_id: Option<u32>,
    ) -> sqlx::Result<u32>;
    async fn update(
        &self,
        id: u32,
        name: Option<&str>,
        active: Option<bool>,
        thumbnail_id: Option<Option<u32>>,
    ) -> sqlx::Result<u32>;
    async fn delete(&self, id: u32) -> sqlx::Result<bool>;
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
        sqlx::query_as::<_, CalendarRow>(r#"SELECT id, name, thumbnail_id, active, created_at, updated_at FROM calendars ORDER BY id DESC"#)
            .fetch_all(&self.pool)
            .await
    }

    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<CalendarRow>> {
        sqlx::query_as::<_, CalendarRow>(r#"SELECT id, name, thumbnail_id, active, created_at, updated_at FROM calendars WHERE id = ?"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn insert(
        &self,
        name: &str,
        active: bool,
        thumbnail_id: Option<u32>,
    ) -> sqlx::Result<u32> {
        let res = sqlx::query!(
            r#"INSERT INTO calendars (name, thumbnail_id, active) VALUES (?, ?, ?)"#,
            name,
            thumbnail_id,
            active
        )
        .execute(&self.pool)
        .await?;
        Ok(res.last_insert_id() as u32)
    }

    async fn update(
        &self,
        id: u32,
        name: Option<&str>,
        active: Option<bool>,
        thumbnail_id: Option<Option<u32>>,
    ) -> sqlx::Result<u32> {
        let res = sqlx::query!(
            r#"
            UPDATE calendars SET
              name = COALESCE(?, name),
              active = COALESCE(?, active),
              thumbnail_id = CASE
                  WHEN ? IS NULL THEN thumbnail_id
                  ELSE ?
              END
            WHERE id = ?
            "#,
            name,
            active,
            thumbnail_id.as_ref().map(|_| 0u8),
            thumbnail_id.unwrap_or(None),
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected() as u32)
    }

    async fn delete(&self, id: u32) -> sqlx::Result<bool> {
        let res = sqlx::query!(r#"DELETE FROM calendars WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
