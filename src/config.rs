use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub http_config: HttpConfig,
    pub db_config: DbConfig,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub port: u16,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub dynamo_endpoint: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let http_config: HttpConfig = Config::builder()
            .add_source(
                config::Environment::with_prefix("HTTP")
                    .try_parsing(true)
                    .prefix_separator("_"),
            )
            .build()
            .and_then(|c| c.try_deserialize())
            .unwrap();

        let db_config: DbConfig = Config::builder()
            .add_source(
                config::Environment::with_prefix("DB")
                    .try_parsing(true)
                    .prefix_separator("_"),
            )
            .build()
            .and_then(|c| c.try_deserialize())
            .unwrap();

        let app_config = Self {
            http_config,
            db_config,
        };
        log::debug!("{:#?}", app_config);
        app_config
    }
}
