use derive_more::Constructor;

use crate::domain::book::book_repo::DynBookRepo;
use crate::domain::book::Book;

pub trait BookUsecase {
    fn get_books(&self) -> Vec<Book>;
}

#[derive(Constructor)]
pub struct BookUsecaseImpl {
    book_repo: DynBookRepo,
}

impl BookUsecase for BookUsecaseImpl {
    fn get_books(&self) -> Vec<Book> {
        self.book_repo.get_books()
    }
}
