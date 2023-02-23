use anyhow::Result;

use crate::config::AppConfig;
use crate::module::Modules;

mod config;
mod domain;
mod infrastructure;
mod module;
mod server;
mod usecase;

async fn try_main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let app_config = AppConfig::load()?;
    let modules = Modules::init().await;
    server::run(modules, app_config.http).await
}

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
