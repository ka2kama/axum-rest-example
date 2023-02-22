use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_config::Credentials;
use axum::{Router, Server};

mod route;

#[inline]
async fn init_app(_port: u16) -> Router {
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

    route::create_route()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let port: u16 = 9000;

    let app = init_app(port).await;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    log::info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}
