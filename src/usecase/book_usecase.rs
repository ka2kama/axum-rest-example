use crate::domain::book::book_repo::BookRepo;
use crate::domain::book::Book;
use async_trait::async_trait;
use derive_more::Constructor;
use im_rc::Vector;
use std::sync::Arc;

#[async_trait]
pub trait BookUsecase {
    async fn get_books(&self) -> anyhow::Result<Vector<Book>>;
}

#[derive(Constructor)]
pub struct BookUsecaseImpl {
    book_repo: Arc<dyn BookRepo + Send + Sync>,
}

#[async_trait]
impl BookUsecase for BookUsecaseImpl {
    async fn get_books(&self) -> anyhow::Result<Vector<Book>> {
        let books = self.book_repo.get_books().await;
        Ok(books)
    }
}
