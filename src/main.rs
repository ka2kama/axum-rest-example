use crate::config::AppConfig;
use crate::module::Modules;

mod config;
mod domain;
mod infrastructure;
mod module;
mod server;
mod usecase;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let AppConfig {
        http_config,
        db_config,
    } = AppConfig::load();
    let modules = Modules::init(db_config).await;
    server::run(modules, http_config).await
}
