use std::sync::Arc;

use axum::{
   extract::{Path, State},
   http::StatusCode,
   response::IntoResponse,
   routing::get,
   Json,
   Router,
};

use crate::usecase::user_usecase::UserUsecase;

type DynUserUsecase = Arc<dyn UserUsecase + Send + Sync>;

pub fn route<S>(user_usecase: DynUserUsecase) -> Router<S>
where
   S: Clone + Send + Sync + 'static,
{
   Router::new()
      .route("/{id}", get(get_user))
      .with_state(user_usecase)
}

async fn get_user(
   Path(id): Path<String>,
   State(user_usecase): State<DynUserUsecase>,
) -> Result<impl IntoResponse, StatusCode> {
   let user = user_usecase
      .get_user(id)
      .await
      .ok_or(StatusCode::NOT_FOUND)?;
   Ok(Json(user))
}
