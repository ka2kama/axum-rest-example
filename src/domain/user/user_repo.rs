use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserRepo {
   async fn get_user(&self, id: String) -> Option<User>;
}
