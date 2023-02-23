use crate::domain::book::Book;

pub trait BookRepo {
    fn get_books(&self) -> Vec<Book>;
}
