use std::sync::Arc;

use aws_config::BehaviorVersion;
use aws_sdk_config::config::Credentials;

use crate::{
   config::DbConfig,
   infrastructure::db::dynamodb::{
      book_repo_for_dynamodb::BookRepoForDynamoDB,
      user_repo_for_dynamodb::UserRepoForDynamoDB,
   },
   usecase::{
      book_usecase::{BookUsecase, BookUsecaseImpl},
      user_usecase::{UserUsecase, UserUsecaseImpl},
   },
};

pub struct Modules {
   pub user_usecase: Arc<dyn UserUsecase + Send + Sync>,
   pub book_usecase: Arc<dyn BookUsecase + Send + Sync>,
}

impl Modules {
   pub async fn init(DbConfig { dynamo_endpoint }: DbConfig) -> anyhow::Result<Self> {
      let config = {
         let mut config_loader =
            aws_config::defaults(BehaviorVersion::latest()).endpoint_url(dynamo_endpoint);
         if cfg!(debug_assertions) {
            config_loader = config_loader
               .credentials_provider(Credentials::new(
                  "dummyKey",
                  "dummySecret",
                  None,
                  None,
                  "dummy-provider",
               ))
               .region("ap-northeast-1");
         }
         config_loader.load()
      }
      .await;

      let dynamodb_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));

      // connectivity test
      {
         let tables_output = dynamodb_client.list_tables().send().await?;
         tracing::info!("{:?}", tables_output.table_names.unwrap_or(vec![]));
      }

      let user_repo = Arc::new(UserRepoForDynamoDB::new(Arc::clone(&dynamodb_client)));
      let book_repo = Arc::new(BookRepoForDynamoDB::new(Arc::clone(&dynamodb_client)));

      let user_usecase = Arc::new(UserUsecaseImpl::new(user_repo));
      let book_usecase = Arc::new(BookUsecaseImpl::new(book_repo));

      Ok(Self {
         user_usecase,
         book_usecase,
      })
   }
}
