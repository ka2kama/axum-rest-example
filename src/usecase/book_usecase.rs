use async_trait::async_trait;
use derive_more::Constructor;

use crate::domain::book::book_repo::DynBookRepo;
use crate::domain::book::Book;

#[async_trait]
pub trait BookUsecase {
    async fn get_books(&self) -> anyhow::Result<Vec<Book>>;
}

#[derive(Constructor)]
pub struct BookUsecaseImpl {
    book_repo: DynBookRepo,
}

#[async_trait]
impl BookUsecase for BookUsecaseImpl {
    async fn get_books(&self) -> anyhow::Result<Vec<Book>> {
        let books = self.book_repo.get_books().await;
        Ok(books)
    }
}
