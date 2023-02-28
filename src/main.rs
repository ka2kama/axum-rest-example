use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::config::AppConfig;
use crate::module::Modules;

mod config;
mod domain;
mod infrastructure;
mod module;
mod server;
mod usecase;

fn init_logger() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .compact();

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();
}

#[tokio::main]
async fn main() {
    init_logger();

    let AppConfig {
        http_config,
        db_config,
    } = AppConfig::load();

    let modules = Modules::init(db_config).await;

    server::run(modules, http_config).await
}
