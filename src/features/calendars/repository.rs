use super::model::CalendarRow;
use async_trait::async_trait;
use sqlx::{MySql, Pool, QueryBuilder};

#[async_trait]
pub trait CalendarsRepository: Send + Sync {
    async fn list(&self) -> sqlx::Result<Vec<CalendarRow>>;
    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<CalendarRow>>;
    async fn get_by_name(&self, name: &str) -> sqlx::Result<Option<CalendarRow>>;
    async fn insert(&self, name: &str, active: bool) -> sqlx::Result<u32>;
    async fn update(&self, id: u32, name: Option<&str>, active: Option<bool>) -> sqlx::Result<u32>;
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
        let rows = sqlx::query!(r#"Select * FROM calendars"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| CalendarRow {
                id: row.id,
                name: row.name,
                active: row.active != 0,
                created_at: row.created_at.clone(),
                updated_at: row.updated_at.clone(),
            })
            .collect())
    }

    async fn get_by_id(&self, id: u32) -> sqlx::Result<Option<CalendarRow>> {
        let row = sqlx::query!(r#"SELECT * FROM calendars WHERE id = ?"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| CalendarRow {
            id: row.id,
            name: row.name,
            active: row.active != 0,
            created_at: row.created_at.clone(),
            updated_at: row.updated_at.clone(),
        }))
    }

    async fn get_by_name(&self, name: &str) -> sqlx::Result<Option<CalendarRow>> {
        let row = sqlx::query!(r#"SELECT * FROM calendars WHERE name = ?"#, name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| CalendarRow {
            id: row.id,
            name: row.name,
            active: row.active != 0,
            created_at: row.created_at.clone(),
            updated_at: row.updated_at.clone(),
        }))
    }

    async fn insert(&self, name: &str, active: bool) -> sqlx::Result<u32> {
        let response = sqlx::query!(
            r#"INSERT INTO calendars (name, active) VALUES (?,?)"#,
            name,
            active
        )
        .execute(&self.pool)
        .await?;

        Ok(response.last_insert_id() as u32)
    }

    async fn update(&self, id: u32, name: Option<&str>, active: Option<bool>) -> sqlx::Result<u32> {
        if name.is_none() && active.is_none() {
            return Ok(0);
        }

        let mut query_builder = QueryBuilder::<MySql>::new("UPDATE calendars SET ");
        let mut set = query_builder.separated(", ");

        if let Some(name) = name {
            set.push("name = ").push_bind(name);
        }
        if let Some(active) = active {
            set.push("active = ").push_bind(active);
        }

        query_builder.push(" WHERE id = ").push_bind(id);

        let result = query_builder.build().execute(&self.pool).await?;

        Ok(result.rows_affected() as u32)
    }

    async fn delete(&self, id: u32) -> sqlx::Result<bool> {
        let result = sqlx::query!(r#"DELETE FROM calendars WHERE id = ?"#, id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
