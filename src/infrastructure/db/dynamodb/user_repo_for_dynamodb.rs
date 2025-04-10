use std::sync::Arc;

use aws_sdk_dynamodb::types::AttributeValue;
use derive_more::Constructor;
use maplit::hashmap;

use crate::{
   domain::user::{User, user_repo::UserRepo},
   infrastructure::db::dynamodb::deserializer::user_deserializer::deserialize_user,
};

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
      let result = req.send().await.expect("request failed");
      result
         .item
         .map(|item| deserialize_user(item).expect("conversion failed"))
   }
}
