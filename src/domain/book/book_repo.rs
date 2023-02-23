use std::sync::Arc;

use crate::domain::book::Book;

pub type DynBookRepo = Arc<dyn BookRepo + Send + Sync>;

pub trait BookRepo {
    fn get_books(&self) -> Vec<Book>;
}
