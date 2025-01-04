use std::sync::Arc;

use derive_more::Constructor;
use im_rc::Vector;

use crate::domain::book::book_repo::BookRepo;
use crate::domain::book::Book;
use crate::infrastructure::db::dynamodb::deserializer::book_deserializer;

#[derive(Constructor)]
pub struct BookRepoForDynamoDB {
   dynamodb_client: Arc<aws_sdk_dynamodb::Client>,
}

#[async_trait::async_trait]
impl BookRepo for BookRepoForDynamoDB {
   async fn get_books(&self) -> Vector<Book> {
      let req = self.dynamodb_client.scan().table_name("books");
      let result = req.send().await.expect("request failed");
      let items = result.items.unwrap_or_default();
      items
         .into_iter()
         .map(book_deserializer::deserialize_book)
         .collect::<Result<_, _>>()
         .expect("convertion failed")
   }
}
