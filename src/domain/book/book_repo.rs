use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::domain::book::Book;

pub type DynBookRepo = Arc<dyn BookRepo + Send + Sync>;

#[async_trait]
pub trait BookRepo {
    async fn get_books(&self) -> Result<Vec<Book>>;
}
