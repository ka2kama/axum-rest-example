use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use axum::body::{Bytes, HttpBody};
use axum::http::{header, HeaderValue, Request};
use axum::{Router, Server};
use tower::{Layer, ServiceBuilder};
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, OnResponse, TraceLayer};
use tower_http::{LatencyUnit, ServiceBuilderExt};
use tracing::error_span;

use crate::config::HttpConfig;
use crate::module::Modules;

mod error;
pub mod route;

#[inline]
pub async fn run(modules: Modules, http_config: HttpConfig) -> Result<()> {
    let app_router = {
        let routes = route::accumulate(modules);
        set_middleware_stack(routes, &http_config)
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], http_config.port));
    log::info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await?;

    Ok(())
}

#[inline]
fn set_middleware_stack<S, B>(app: Router<S, B>, http_config: &HttpConfig) -> Router<S, B>
where
    S: Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
{
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();
    let middleware_stack = ServiceBuilder::new()
        // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
        .sensitive_request_headers(sensitive_headers.clone())
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
        // Return an error after 30 seconds
        .layer(TimeoutLayer::new(Duration::from_secs(
            http_config.timeout_seconds,
        )))
        // Box the response body so it implements `Default` which is required by axum
        .map_response_body(axum::body::boxed)
        // Compress response bodies
        .compression()
        // Set a `Content-Type` if there isn't one already.
        .insert_response_header_if_not_present(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        )
        .into_inner();

    app.layer(middleware_stack)
}
