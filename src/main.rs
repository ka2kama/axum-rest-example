use im_rc::Vector;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::AppConfig, module::Modules};

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

async fn try_main(_args: Vector<String>) -> anyhow::Result<()> {
   init_logger();

   let AppConfig {
      http_config,
      db_config,
   } = AppConfig::load()?;

   let modules = Modules::init(db_config).await?;

   server::run(modules, http_config).await?;

   Ok(())
}

#[tokio::main]
async fn main() {
   let args: Vector<String> = std::env::args().collect();
   if let Err(err) = try_main(args).await {
      tracing::error!("{:#}", err);
      panic!("{:?}", err);
   }
}
