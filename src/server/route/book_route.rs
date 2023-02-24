use std::sync::Arc;

use crate::server::error::AppError;
use axum::body::HttpBody;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::usecase::book_usecase::BookUsecase;

type DynBookUsecase = Arc<dyn BookUsecase + Send + Sync>;

pub fn route<S, B>(book_usecase: DynBookUsecase) -> Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(books_index))
        .with_state(book_usecase)
}

async fn books_index(book_usecase: State<DynBookUsecase>) -> Result<impl IntoResponse, AppError> {
    let books = book_usecase.get_books().await?;
    Ok(Json(books))
}
