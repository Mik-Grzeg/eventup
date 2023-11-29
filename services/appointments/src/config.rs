use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    // postgresql settings
    pub pg_url: String,
    pub pg_max_conn: u32,

    // access control api url
    pub access_control_url: String,
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
            access_control_url: "http://users-api:8080/api/v1/auth/access".into()
        }
    }
}
