use std::sync::Arc;

use derive_more::Constructor;
use serde_dynamo::from_items;

use crate::domain::book::book_repo::BookRepo;
use crate::domain::book::Book;

#[derive(Constructor)]
pub struct BookRepoForDynamoDB {
    dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

#[async_trait::async_trait]
impl BookRepo for BookRepoForDynamoDB {
    async fn get_books(&self) -> anyhow::Result<Vec<Book>> {
        let req = self.dynamodb_client.scan().table_name("books");
        let result = req.send().await?;
        let books = match result.items {
            Some(items) if !items.is_empty() => from_items(items).unwrap(),
            _ => vec![],
        };
        Ok(books)
    }
}
