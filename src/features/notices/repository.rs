use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::features::notices::model::NoticeRow;

#[async_trait]
pub trait NoticesRepository: Send + Sync {
    async fn list(&self) -> sqlx::Result<Vec<NoticeRow>>;
    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<NoticeRow>>;
    async fn insert(&self, title: &str, content: &str, active: bool) -> sqlx::Result<u32>;
    async fn update(
        &self,
        id: u32,
        title: Option<&str>,
        content: Option<&str>,
        active: Option<bool>,
    ) -> sqlx::Result<u32>;
    async fn delete(&self, id: u32) -> sqlx::Result<bool>;
}

pub type DynamicNoticesRepository = std::sync::Arc<dyn NoticesRepository>;

#[derive(Clone)]
pub struct MySqlNoticesRepository {
    pool: Pool<MySql>,
}

impl MySqlNoticesRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NoticesRepository for MySqlNoticesRepository {
    async fn list(&self) -> sqlx::Result<Vec<NoticeRow>> {
        let rows = sqlx::query!(r#"SELECT id, title, content, active, created_at FROM notices"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| NoticeRow {
                id: row.id,
                title: row.title,
                content: row.content,
                active: row.active != 0,
                created_at: row.created_at.clone(),
            })
            .collect())
    }

    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<NoticeRow>> {
        let row = sqlx::query!(
            r#"SELECT id, title, content, active, created_at FROM notices WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| NoticeRow {
            id: row.id,
            title: row.title,
            content: row.content,
            active: row.active != 0,
            created_at: row.created_at.clone(),
        }))
    }

    async fn insert(&self, title: &str, content: &str, active: bool) -> sqlx::Result<u32> {
        let result = sqlx::query!(
            r#"INSERT INTO notices (title, content, active) VALUES (?, ?, ?)"#,
            title,
            content,
            active
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    async fn update(
        &self,
        id: u32,
        title: Option<&str>,
        content: Option<&str>,
        active: Option<bool>,
    ) -> sqlx::Result<u32> {
        if title.is_none() && content.is_none() && active.is_none() {
            return Ok(0);
        }

        let result = sqlx::query!(
            r#"UPDATE notices SET title = COALESCE(?, title), content = COALESCE(?, content), active = COALESCE(?, active) WHERE id = ?"#,
            title,
            content,
            active,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as u32)
    }

    async fn delete(&self, id: u32) -> sqlx::Result<bool> {
        let result = sqlx::query!(r#"DELETE FROM notices WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
