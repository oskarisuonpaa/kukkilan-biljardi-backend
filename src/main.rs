mod config;
mod error;
mod features;
mod infrastructure;
mod response;
mod state;

use axum::{Router, http::HeaderValue};
use config::AppConfig;
use infrastructure::database::connect;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::{
    infrastructure::database::run_migrations, 
    state::AppState,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env();
    let pool = connect(&config.database_url).await;
    run_migrations(&pool).await;

    let app_state = AppState::new(config.clone(), pool);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let uploads_service = ServeDir::new(app_state.config.media_root.clone());

    let app = Router::new()
        // Public routes
        .merge(features::calendars::routes())
        .merge(features::bookings::routes())
        .merge(features::notices::routes())
        .merge(features::opening_hours::routes())
        .merge(features::contact_info::routes())
        .merge(features::media::routes())
        // Auth routes
        .merge(features::auth::routes())
        .nest_service(&app_state.config.media_base_url, uploads_service)
        .with_state(app_state)
        .layer(cors);

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
