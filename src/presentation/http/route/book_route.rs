use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::domain::book::Book;

pub struct BookRoute {}

impl BookRoute {
    pub fn route() -> Router {
        Router::new().route("/", get(Self::books_index))
    }

    async fn books_index() -> impl IntoResponse {
        let books = vec![Book::new("1".to_string(), "rust".to_string())];
        Json(books)
    }
}
