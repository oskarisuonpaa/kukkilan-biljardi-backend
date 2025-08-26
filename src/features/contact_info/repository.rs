use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::features::contact_info::model::ContactInfoRow;

#[async_trait]
pub trait ContactInfoRepository: Send + Sync {
    async fn get(&self) -> sqlx::Result<Option<ContactInfoRow>>;
    async fn set(&self, address: &str, phone: &str, email: &str) -> sqlx::Result<()>;
}

pub type DynamicContactInfoRepository = std::sync::Arc<dyn ContactInfoRepository>;

#[derive(Clone)]
pub struct MySqlContactInfoRepository {
    pool: Pool<MySql>,
}

impl MySqlContactInfoRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactInfoRepository for MySqlContactInfoRepository {
    async fn get(&self) -> sqlx::Result<Option<ContactInfoRow>> {
        let row = sqlx::query!(
            r#"SELECT id, address, phone, email, updated_at FROM contact_info WHERE id = 1"#
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| ContactInfoRow {
            id: row.id as u32,
            address: row.address,
            phone: row.phone,
            email: row.email,
            updated_at: row.updated_at.naive_utc(),
        }))
    }

    async fn set(&self, address: &str, phone: &str, email: &str) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO contact_info (id, address, phone, email)
            VALUES (1, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                address = VALUES(address),
                phone = VALUES(phone),
                email = VALUES(email)
            "#,
            address,
            phone,
            email
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
