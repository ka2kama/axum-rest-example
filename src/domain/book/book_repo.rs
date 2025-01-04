use im_rc::Vector;

use crate::domain::book::Book;

#[async_trait::async_trait]
pub trait BookRepo {
   async fn get_books(&self) -> Vector<Book>;
}
