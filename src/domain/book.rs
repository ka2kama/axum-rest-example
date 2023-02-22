use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Constructor)]
pub struct Book {
    id: String,
    title: String,
}
