use derive_more::Constructor;
use serde::{Deserialize, Serialize};

pub mod book_repo;

#[derive(Debug, Deserialize, Serialize, Clone, Constructor)]
pub struct Book {
    id: String,
    title: String,
}
