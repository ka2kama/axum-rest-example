use axum::{routing::get, Router};

pub fn route() -> Router {
   Router::new().route("/", get(handler))
}

async fn handler() {}
