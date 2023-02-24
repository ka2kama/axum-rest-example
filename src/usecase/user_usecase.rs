use std::sync::Arc;

use derive_more::Constructor;

use crate::domain::user::user_repo::UserRepo;
use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserUsecase {
    async fn get_user(&self, id: String) -> Option<User>;
}

#[derive(Constructor)]
pub struct UserUsecaseImpl {
    user_repo: Arc<dyn UserRepo + Send + Sync>,
}

#[async_trait::async_trait]
impl UserUsecase for UserUsecaseImpl {
    async fn get_user(&self, id: String) -> Option<User> {
        self.user_repo.get_user(id).await
    }
}
