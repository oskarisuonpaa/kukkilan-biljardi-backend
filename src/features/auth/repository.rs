use sqlx::{MySqlPool, Row};
use chrono::Utc;
use async_trait::async_trait;
use crate::features::auth::model::AdminUser;
use crate::error::AppError;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError>;
    async fn update_last_login(&self, user_id: &str) -> Result<(), AppError>;
}

pub struct MySqlAuthRepository {
    pool: MySqlPool,
}

impl MySqlAuthRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for MySqlAuthRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError> {
        let query = sqlx::query_as::<_, AdminUser>(
            r#"
            SELECT id, username, password_hash, email, is_active, created_at, last_login
            FROM admin_users
            WHERE username = ? AND is_active = TRUE
            "#,
        )
        .bind(username);

        match query.fetch_optional(&self.pool).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::error!("Failed to find admin user by username: {}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    async fn update_last_login(&self, user_id: &str) -> Result<(), AppError> {
        let query = sqlx::query(
            r#"
            UPDATE admin_users 
            SET last_login = ?
            WHERE id = ?
            "#,
        )
        .bind(Utc::now())
        .bind(user_id);

        match query.execute(&self.pool).await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Failed to update last login: {}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }
}