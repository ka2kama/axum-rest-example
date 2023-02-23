use derive_more::Constructor;

use crate::domain::book::book_repo::BookRepo;
use crate::domain::book::Book;

pub trait BookUsecase {
    fn get_books(&self) -> Vec<Book>;
}

#[derive(Constructor)]
pub struct BookUsecaseImpl<T: BookRepo> {
    book_repo: T,
}

impl<T: BookRepo> BookUsecase for BookUsecaseImpl<T> {
    fn get_books(&self) -> Vec<Book> {
        self.book_repo.get_books()
    }
}
