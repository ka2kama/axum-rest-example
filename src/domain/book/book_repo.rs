use crate::domain::book::Book;
use im_rc::Vector;

#[async_trait::async_trait]
pub trait BookRepo {
    async fn get_books(&self) -> Vector<Book>;
}
