use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use aws_sdk_config::Credentials;
use axum::{Router, Server};

use crate::config::AppConfig;

mod config;
mod route;

#[inline]
async fn init_app(app_config: &AppConfig) -> Result<Router> {
    let config = aws_config::from_env()
        .endpoint_url("http://localhost:8000")
        .credentials_provider(Credentials::new(
            "dummy-key",
            "dummy-secret",
            None,
            None,
            "dummy-provider",
        ))
        .region("ap-northeast-1")
        .load()
        .await;
    let _dynamodb_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));

    let router = route::create_route(app_config);
    Ok(router)
}

async fn try_main() -> Result<()> {
    let app_config = AppConfig::load()?;
    let app = init_app(&app_config).await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], app_config.http.port));
    log::info!("listening on {addr}");
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(e) = try_main().await {
        log::error!("{:?}", e);
        std::process::exit(1);
    }
}
