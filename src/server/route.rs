use axum::{response::Html, routing::get, Router};

use crate::module::Modules;

pub mod book_route;
pub mod health_check_route;
pub mod user_route;

pub fn accumulate(
   Modules {
      user_usecase,
      book_usecase,
   }: Modules,
) -> Router {
   Router::new()
      .route("/", get(|| async { Html("<h1>hello world!</h1>") }))
      .nest("/health", health_check_route::route())
      .nest("/users", user_route::route(user_usecase))
      .nest("/books", book_route::route(book_usecase))
}
