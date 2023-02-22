use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub http: HttpConfig,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub port: u16,
    pub timeout_seconds: u64,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let http: HttpConfig = Config::builder()
            .add_source(
                config::Environment::with_prefix("HTTP")
                    .try_parsing(true)
                    .prefix_separator("_")
                    .list_separator(" "),
            )
            .build()
            .and_then(|c| c.try_deserialize())?;

        let app_config = Self { http };
        log::debug!("{:#?}", app_config);
        Ok(app_config)
    }
}
