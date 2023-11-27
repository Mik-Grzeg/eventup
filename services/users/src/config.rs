use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    // postgresql settings
    pub pg_url: String,
    pub pg_max_conn: u32,
}

impl AppConfig {
    pub fn new() -> Self {
        let config = Config::builder()
            .add_source(config::Environment::with_prefix("RUST"))
            .build()
            .unwrap();

        tracing::debug!("Config: {config:?}");

        config.try_deserialize().unwrap()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            pg_url: "postgres://postgres:password@localhost:5432/".into(),
            pg_max_conn: 5,
        }
    }
}
