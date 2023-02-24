use std::sync::Arc;

use aws_sdk_dynamodb::model::AttributeValue;
use derive_more::Constructor;
use maplit::hashmap;
use serde_dynamo::from_item;

use crate::domain::user::user_repo::UserRepo;
use crate::domain::user::User;

#[derive(Constructor)]
pub struct UserRepoForDynamoDB {
    dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

#[async_trait::async_trait]
impl UserRepo for UserRepoForDynamoDB {
    async fn get_user(&self, id: String) -> Option<User> {
        let req = self
            .dynamodb_client
            .get_item()
            .table_name("users")
            .set_key(Some(hashmap! {
                "id".to_owned() => AttributeValue::S(id)
            }));
        let result = req.send().await.unwrap();
        match result.item {
            Some(item) => from_item(item).unwrap(),
            None => None,
        }
    }
}
