use std::any::Any;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub fn handle_panic(e: Box<dyn Any + Send + 'static>) -> Response {
   if let Ok(e) = e.downcast::<String>() {
      tracing::error!("{}", e);
   }
   (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
   fn into_response(self) -> Response {
      tracing::error!("{:?}", self.0);
      (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
   }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
   E: Into<anyhow::Error>,
{
   fn from(err: E) -> Self {
      Self(err.into())
   }
}
