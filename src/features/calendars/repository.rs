use super::model::CalendarRow;
use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[async_trait]
pub trait CalendarsRepository: Send + Sync {
    async fn list(&self) -> sqlx::Result<Vec<CalendarRow>>;
    async fn get_by_id(&self, id: u64) -> sqlx::Result<Option<CalendarRow>>;
    async fn get_by_name(&self, name: &str) -> sqlx::Result<Option<CalendarRow>>;
    async fn insert(&self, name: &str, active: bool) -> sqlx::Result<u64>;
    async fn update(&self, id: u64, name: Option<&str>, active: Option<bool>)
    -> sqlx::Result<bool>;
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

    async fn get_by_id(&self, id: u64) -> sqlx::Result<Option<CalendarRow>> {
        let row = sqlx::query!(r#"SELECT * FROM calendars WHERE id = ?"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| CalendarRow {
            id: row.id,
            name: row.name,
            active: row.active != 0,
            created_at: row.created_at.naive_utc(),
            updated_at: row.updated_at.naive_utc(),
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
            created_at: row.created_at.naive_utc(),
            updated_at: row.updated_at.naive_utc(),
        }))
    }

    async fn insert(&self, name: &str, active: bool) -> sqlx::Result<u64> {
        let response = sqlx::query!(
            r#"INSERT INTO calendars (name, active) VALUES (?,?)"#,
            name,
            active
        )
        .execute(&self.pool)
        .await?;

        Ok(response.last_insert_id())
    }

    async fn update(
        &self,
        id: u64,
        name: Option<&str>,
        active: Option<bool>,
    ) -> sqlx::Result<bool> {
        let mut sql = String::from("UPDATE calendars SET ");
        let mut set_parts = vec![];

        if name.is_some() {
            set_parts.push("name = ?");
        }
        if active.is_some() {
            set_parts.push("active = ?");
        }

        if set_parts.is_empty() {
            return Ok(false);
        }

        sql.push_str(&set_parts.join(", "));
        sql.push_str(" WHERE id = ?");

        let mut query = sqlx::query(&sql);

        if let Some(name) = name {
            query = query.bind(name);
        }
        if let Some(active) = active {
            query = query.bind(active);
        }
        query = query.bind(id);

        let res = query.execute(&self.pool).await?;
        Ok(res.rows_affected() > 0)
    }
}
