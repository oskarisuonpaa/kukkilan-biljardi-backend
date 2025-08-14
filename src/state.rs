use crate::config::AppConfig;
use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: Pool<MySql>,
}

impl AppState {
    pub fn new(config: AppConfig, pool: Pool<MySql>) -> Self {
        Self { config, pool }
    }
}
