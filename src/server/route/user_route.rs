use std::sync::Arc;

use axum::body::HttpBody;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::usecase::user_usecase::UserUsecase;

type DynUserUsecase = Arc<dyn UserUsecase + Send + Sync>;

pub fn route<S, B>(user_usecase: DynUserUsecase) -> Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/:id", get(get_user))
        .with_state(user_usecase)
}

async fn get_user(user_usecase: State<DynUserUsecase>) -> impl IntoResponse {
    let user = user_usecase.get_user();
    Json(user)
}
