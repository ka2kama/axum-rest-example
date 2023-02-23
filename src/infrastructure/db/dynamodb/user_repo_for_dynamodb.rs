use std::sync::Arc;

use derive_more::Constructor;

use crate::domain::user::user_repo::UserRepo;
use crate::domain::user::User;

#[derive(Constructor)]
pub struct UserRepoForDynamoDB {
    _dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

impl UserRepo for UserRepoForDynamoDB {
    fn get_user(&self) -> Option<User> {
        None
    }
}
