use axum::body::HttpBody;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::usecase::book_usecase::DynBookUsecase;

pub fn route<S, B>(book_usecase: DynBookUsecase) -> Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(books_index))
        .with_state(book_usecase)
}

async fn books_index(book_usecase: State<DynBookUsecase>) -> impl IntoResponse {
    let books = book_usecase.get_books();
    Json(books)
}
