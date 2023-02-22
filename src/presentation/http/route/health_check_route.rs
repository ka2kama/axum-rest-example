use axum::routing::get;
use axum::Router;

pub struct HealthCheckRoute;

impl HealthCheckRoute {
    pub fn route() -> Router {
        Router::new().route("/", get(Self::handler))
    }

    async fn handler() {}
}
