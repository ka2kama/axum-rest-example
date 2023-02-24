use std::sync::Arc;

use crate::domain::book::Book;

pub type DynBookRepo = Arc<dyn BookRepo + Send + Sync>;

#[async_trait::async_trait]
pub trait BookRepo {
    async fn get_books(&self) -> anyhow::Result<Vec<Book>>;
}
