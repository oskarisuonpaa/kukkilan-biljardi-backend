use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

pub async fn connect(url: &str) -> Pool<MySql> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .expect("database connect failed")
}

pub async fn run_migrations(pool: &Pool<MySql>) {
    // Temporarily disable migrations to fix checksum issues
    // sqlx::migrate!("./migrations")
    //     .run(pool)
    //     .await
    //     .expect("migrations failed");
    
    tracing::info!("Migrations skipped - manual setup in place");
}
