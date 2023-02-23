use std::net::SocketAddr;

use anyhow::Result;
use axum::Server;

use crate::config::HttpConfig;
use crate::module::Modules;

pub mod route;

#[inline]
pub async fn run(modules: Modules, http_config: HttpConfig) -> Result<()> {
    let app_router = route::create(modules, &http_config);
    let addr = SocketAddr::from(([127, 0, 0, 1], http_config.port));
    log::info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await?;

    Ok(())
}
