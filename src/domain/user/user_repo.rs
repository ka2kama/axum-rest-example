use crate::domain::user::User;

pub trait UserRepo {
    fn get_user(&self) -> Option<User>;
}
