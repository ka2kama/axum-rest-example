use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};

use crate::{server::error::AppError, usecase::book_usecase::BookUsecase};

type DynBookUsecase = Arc<dyn BookUsecase + Send + Sync>;

pub fn route<S>(book_usecase: DynBookUsecase) -> Router<S>
where
   S: Clone + Send + Sync + 'static,
{
   Router::new()
      .route("/", get(books_index))
      .with_state(book_usecase)
}

async fn books_index(
   State(book_usecase): State<DynBookUsecase>,
) -> Result<impl IntoResponse, AppError> {
   let books = book_usecase.get_books().await?;
   Ok(Json(books))
}
