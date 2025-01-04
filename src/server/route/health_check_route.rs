use axum::routing::get;
use axum::Router;

pub fn route() -> Router {
   Router::new().route("/", get(handler))
}

async fn handler() {}
