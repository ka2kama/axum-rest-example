use crate::config::AppConfig;
use crate::module::Modules;

mod config;
mod domain;
mod infrastructure;
mod module;
mod server;
mod usecase;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app_config = AppConfig::load()?;
    let modules = Modules::init().await;
    server::run(modules, app_config.http).await
}
