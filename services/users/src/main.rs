use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

use crate::{app_state::AppState, config::AppConfig};

mod app_state;
mod config;
mod handlers;
mod repository;
mod types;

pub fn init_tracing() {
    let logger = tracing_subscriber::fmt::layer().compact();
    let env_filter = EnvFilter::try_from_default_env()
        .or(EnvFilter::try_new("info"))
        .unwrap();

    let collector = Registry::default().with(logger).with(env_filter);

    tracing::subscriber::set_global_default(collector).unwrap();
}

#[tokio::main]
async fn main() {
    // Set up tracing
    init_tracing();

    let config = AppConfig::new();
    let app_state = AppState::from(&config).await;

    // Build application with routes
    let app = handlers::router::router(app_state);

    // Run the app with hyper, listening globally on port 8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server is listening on {} address", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start.");
}
