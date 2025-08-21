use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::features::notices::model::NoticeRow;

#[async_trait]
pub trait NoticesRepository: Send + Sync {
    async fn list(&self) -> sqlx::Result<Vec<NoticeRow>>;
    // async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<NoticeRow>>;
    // async fn insert(&self, title: &str, content: &str, active: bool) -> sqlx::Result<u32>;
    // async fn update(
    //     &self,
    //     id: u32,
    //     title: Option<&str>,
    //     content: Option<&str>,
    //     active: Option<bool>,
    // ) -> sqlx::Result<u32>;
    // async fn delete(&self, id: u32) -> sqlx::Result<bool>;
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
        let rows = sqlx::query!(r#"Select id, title, content, active, created_at FROM notices"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| NoticeRow {
                id: row.id,
                title: row.title,
                content: row.content,
                active: row.active,
                created_at: row.created_at.clone(),
            })
            .collect())
    }
}
