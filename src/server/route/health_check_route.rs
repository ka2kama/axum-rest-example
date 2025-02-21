use axum::{Router, routing::get};

pub fn route() -> Router {
   Router::new().route("/", get(handler))
}

async fn handler() {}
