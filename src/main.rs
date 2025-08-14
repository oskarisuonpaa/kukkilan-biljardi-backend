mod config;
mod error;
mod features;
mod infrastructure;
mod state;

use axum::Router;
use config::AppConfig;
use infrastructure::database::connect;

use crate::{infrastructure::database::run_migrations, state::AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env();
    let pool = connect(&config.database_url).await;
    run_migrations(&pool).await;

    let app_state = AppState::new(config.clone(), pool);

    let app = Router::new()
        .merge(features::calendars::routes())
        .with_state(app_state);

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
