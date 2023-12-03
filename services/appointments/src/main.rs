use appointments_lib::{app_state::AppState, config::AppConfig, handlers, init_tracing};

#[tokio::main]
async fn main() {
    // Set up tracing
    init_tracing();

    let config = AppConfig::new();
    let app_state = AppState::from_config(&config).await;

    // Build application with routes
    let app = handlers::router::router(app_state);

    // Listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server is listening on {:?}", listener);

    axum::serve(listener, app)
        .await
        .expect("Server failed to start.");
}
