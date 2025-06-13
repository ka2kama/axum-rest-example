use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
   Router,
   http::{HeaderValue, Request, header},
};
use chrono::Utc;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
   LatencyUnit,
   ServiceBuilderExt,
   catch_panic::CatchPanicLayer,
   request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
   timeout::TimeoutLayer,
   trace::{DefaultOnResponse, TraceLayer},
};
use tracing::error_span;
use uuid::Uuid;

use crate::{config::HttpConfig, module::Modules};

mod error;
pub mod route;

pub async fn run(modules: Modules, http_config: HttpConfig) -> anyhow::Result<()> {
   let app_router = {
      let routes = route::accumulate(modules);
      set_middleware_stack(routes, &http_config)
   };

   let ipv4 = if cfg!(debug_assertions) {
      [127, 0, 0, 1]
   } else {
      [0, 0, 0, 0]
   };
   let addr = SocketAddr::from((ipv4, http_config.port));
   let listener = tokio::net::TcpListener::bind(addr).await?;
   tracing::info!("listening on http://{}", listener.local_addr()?);

   axum::serve(listener, app_router)
      .with_graceful_shutdown(shutdown_signal())
      .await?;

   Ok(())
}

#[inline]
fn set_middleware_stack<S>(app: Router<S>, http_config: &HttpConfig) -> Router<S>
where
   S: Clone + Send + Sync + 'static,
{
   let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();
   let middleware_stack = ServiceBuilder::new()
      // mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
      .sensitive_request_headers(sensitive_headers.clone())
      // set `x-request-id` header on all requests
      .layer(SetRequestIdLayer::x_request_id(MakeRequestSimpleUuid))
      // propagate `x-request-id` headers from request to response
      .layer(PropagateRequestIdLayer::x_request_id())
      .layer(
         // create a tracing span for each request
         TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
               // get the request id from the extensions
               let extensions = request.extensions();
               let request_id = extensions
                  .get::<RequestId>()
                  .and_then(|id| id.header_value().to_str().ok())
                  .unwrap_or("unknown");
               // put it along with other information into the `request` span
               error_span!("", request_id = request_id)
            })
            .on_response(
               DefaultOnResponse::new()
                  .include_headers(true)
                  .latency_unit(LatencyUnit::Micros),
            ),
      )
      .layer(CatchPanicLayer::custom(error::handle_panic))
      .sensitive_response_headers(sensitive_headers)
      // Return an error after timeout seconds
      .layer(TimeoutLayer::new(Duration::from_secs(
         http_config.timeout_seconds,
      )))
      // Box the response body so it implements `Default` which is required by axum
      .map_response_body(axum::body::Body::new)
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
async fn shutdown_signal() {
   let ctrl_c = async {
      signal::ctrl_c()
         .await
         .expect("failed to install Ctrl+C handler");
   };

   #[cfg(unix)]
   let terminate = async {
      signal::unix::signal(signal::unix::SignalKind::terminate())
         .expect("failed to install signal handler")
         .recv()
         .await;
   };

   #[cfg(not(unix))]
   let terminate = std::future::pending::<()>();

   tokio::select! {
       _ = ctrl_c => {},
       _ = terminate => {},
   }

   tracing::warn!("signal received, starting graceful shutdown");
}

#[derive(Clone, Copy)]
pub struct MakeRequestSimpleUuid;

impl MakeRequestId for MakeRequestSimpleUuid {
   fn make_request_id<B>(&mut self, req: &Request<B>) -> Option<RequestId> {
      let request_id = match req.headers().get("x-request-id") {
         Some(request_id) if !request_id.is_empty() => request_id.to_owned(),
         _ => {
            let request_id = format!(
               "{}_{}",
               Uuid::new_v4().simple(),
               Utc::now().timestamp_millis()
            );
            request_id.parse().unwrap()
         }
      };
      Some(RequestId::new(request_id))
   }
}
