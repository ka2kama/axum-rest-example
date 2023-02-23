use std::sync::Arc;

use aws_sdk_config::Credentials;

use crate::infrastructure::db::dynamodb::book_repo_for_dynamodb::BookRepoForDynamoDB;
use crate::usecase::book_usecase::{BookUsecase, BookUsecaseImpl};

pub struct Modules {
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
        let book_repo = BookRepoForDynamoDB::new(dynamodb_client);
        let book_usecase = BookUsecaseImpl::new(book_repo);

        Self {
            book_usecase: Arc::new(book_usecase),
        }
    }
}
