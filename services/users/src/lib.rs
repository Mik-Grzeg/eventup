use tracing_subscriber::prelude::*;

pub mod app_state;
pub mod config;
pub mod handlers;
pub mod middlewares;
pub mod repository;
pub mod types;

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lib=debug,tower_http=trace,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
