use std::sync::Arc;

use derive_more::Constructor;

use crate::domain::user::user_repo::UserRepo;
use crate::domain::user::User;

pub trait UserUsecase {
    fn get_user(&self) -> Option<User>;
}

#[derive(Constructor)]
pub struct UserUsecaseImpl {
    user_repo: Arc<dyn UserRepo + Send + Sync>,
}

impl UserUsecase for UserUsecaseImpl {
    fn get_user(&self) -> Option<User> {
        self.user_repo.get_user()
    }
}
