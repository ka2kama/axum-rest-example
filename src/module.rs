use std::sync::Arc;

use aws_sdk_config::Credentials;

use crate::infrastructure::db::dynamodb::book_repo_for_dynamodb::UserRepoForDynamoDB;
use crate::infrastructure::db::dynamodb::user_repo_for_dynamodb::BookRepoForDynamoDB;
use crate::usecase::book_usecase::{BookUsecase, BookUsecaseImpl};
use crate::usecase::user_usecase::{UserUsecase, UserUsecaseImpl};

pub struct Modules {
    pub user_usecase: Arc<dyn UserUsecase + Send + Sync>,
    pub book_usecase: Arc<dyn BookUsecase + Send + Sync>,
}

impl Modules {
    pub async fn init() -> Self {
        let config = aws_config::from_env()
            .endpoint_url("http://localhost:8000")
            .credentials_provider(Credentials::new(
                "dummy-key",
                "dummy-secret",
                None,
                None,
                "dummy-provider",
            ))
            .region("ap-northeast-1")
            .load()
            .await;
        let dynamodb_client = Arc::new(aws_sdk_dynamodb::Client::new(&config));
        let user_repo = Arc::new(UserRepoForDynamoDB::new(Arc::clone(&dynamodb_client)));
        let book_repo = Arc::new(BookRepoForDynamoDB::new(Arc::clone(&dynamodb_client)));

        let user_usecase = Arc::new(UserUsecaseImpl::new(user_repo));
        let book_usecase = Arc::new(BookUsecaseImpl::new(book_repo));

        Self {
            user_usecase,
            book_usecase,
        }
    }
}
