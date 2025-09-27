use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct AdminUser {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSession {
    pub user_id: String,
    pub username: String,
    pub login_time: DateTime<Utc>,
}
