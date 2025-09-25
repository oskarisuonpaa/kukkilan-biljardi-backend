use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub media_root: std::path::PathBuf,
    pub media_base_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            media_root: std::env::var("MEDIA_ROOT")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| std::path::PathBuf::from("storage/uploads")),
            media_base_url: std::env::var("MEDIA_BASE_URL")
                .unwrap_or_else(|_| "/uploads".to_string()),
        }
    }
}
