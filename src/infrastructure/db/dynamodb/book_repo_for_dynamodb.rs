use std::sync::Arc;

use derive_more::Constructor;

use crate::domain::book::book_repo::BookRepo;
use crate::domain::book::Book;

#[derive(Constructor)]
pub struct BookRepoForDynamoDB {
    _dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

impl BookRepo for BookRepoForDynamoDB {
    fn get_books(&self) -> Vec<Book> {
        vec![Book::new("1".to_string(), "rust".to_string())]
    }
}
