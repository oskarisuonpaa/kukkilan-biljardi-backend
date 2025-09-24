use super::model::MediaRow;
use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn list(&self, only_backgrounds: Option<bool>) -> sqlx::Result<Vec<MediaRow>>;
    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<MediaRow>>;
    async fn insert(
        &self,
        is_background: bool,
        kind: &str,
        file_url: &str,
        alt_text: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> sqlx::Result<u32>;
    async fn update(
        &self,
        id: u32,
        is_background: Option<bool>,
        kind: Option<&str>,
        file_url: Option<&str>,
        alt_text: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> sqlx::Result<u32>;
    async fn delete(&self, id: u32) -> sqlx::Result<bool>;
}

pub type DynamicMediaRepository = std::sync::Arc<dyn MediaRepository>;

pub struct MySqlMediaRepository {
    pool: Pool<MySql>,
}

impl MySqlMediaRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MediaRepository for MySqlMediaRepository {
    async fn list(&self, only_backgrounds: Option<bool>) -> sqlx::Result<Vec<MediaRow>> {
        match only_backgrounds {
            Some(true) => {
                sqlx::query_as::<_, MediaRow>(
                    r#"SELECT * FROM media WHERE is_background = TRUE ORDER BY id DESC"#,
                )
                .fetch_all(&self.pool)
                .await
            }
            Some(false) => {
                sqlx::query_as::<_, MediaRow>(
                    r#"SELECT * FROM media WHERE is_background = FALSE ORDER BY id DESC"#,
                )
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query_as::<_, MediaRow>(r#"SELECT * FROM media ORDER BY id DESC"#)
                    .fetch_all(&self.pool)
                    .await
            }
        }
    }

    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<MediaRow>> {
        sqlx::query_as::<_, MediaRow>(r#"SELECT * FROM media WHERE id = ?"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn insert(
        &self,
        is_background: bool,
        kind: &str,
        file_url: &str,
        alt_text: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> sqlx::Result<u32> {
        let res = sqlx::query!(
            r#"
            INSERT INTO media (is_background, kind, file_url, alt_text, width, height)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            is_background,
            kind,
            file_url,
            alt_text,
            width,
            height
        )
        .execute(&self.pool)
        .await?;

        Ok(res.last_insert_id() as u32)
    }

    async fn update(
        &self,
        id: u32,
        is_background: Option<bool>,
        kind: Option<&str>,
        file_url: Option<&str>,
        alt_text: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> sqlx::Result<u32> {
        let res = sqlx::query!(
            r#"
            UPDATE media SET
              is_background = COALESCE(?, is_background),
              kind          = COALESCE(?, kind),
              file_url      = COALESCE(?, file_url),
              alt_text      = COALESCE(?, alt_text),
              width         = COALESCE(?, width),
              height        = COALESCE(?, height)
            WHERE id = ?
            "#,
            is_background,
            kind,
            file_url,
            alt_text,
            width,
            height,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(res.rows_affected() as u32)
    }

    async fn delete(&self, id: u32) -> sqlx::Result<bool> {
        let res = sqlx::query!(r#"DELETE FROM media WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
