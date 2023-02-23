use std::borrow::Cow;
use std::time::Duration;

use axum::body::HttpBody;
use axum::http::Request;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tower_http::ServiceBuilderExt;
use tracing::error_span;

use crate::config::HttpConfig;
use crate::module::Modules;

pub mod book_route;
pub mod health_check_route;

pub fn create(modules: Modules, http_config: &HttpConfig) -> Router {
    let app = Router::new()
        .nest("/health", health_check_route::route())
        .nest("/books", book_route::route(modules.book_usecase));
    set_middleware_stack(app, http_config)
}

#[inline]
fn set_middleware_stack<S, B>(app: Router<S, B>, http_config: &HttpConfig) -> Router<S, B>
where
    S: Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
{
    let middleware_stack = ServiceBuilder::new()
        // set `x-request-id` header on all requests
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        // propagate `x-request-id` headers from request to response
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            // Let's create a tracing span for each request
            TraceLayer::new_for_http().make_span_with(|request: &Request<B>| {
                // We get the request id from the extensions
                let request_id: Cow<'static, str> = match request
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id| id.header_value().to_str().ok())
                {
                    Some(request_id) => request_id.replace("-", "").into(),
                    None => "unknown".into(),
                };
                // And then we put it along with other information into the `request` span
                error_span!("request", id = request_id.as_ref())
            }),
        )
        // Compress response bodies
        .compression()
        // Return an error after 30 seconds
        .layer(TimeoutLayer::new(Duration::from_secs(
            http_config.timeout_seconds,
        )))
        .into_inner();

    app.layer(middleware_stack)
}
