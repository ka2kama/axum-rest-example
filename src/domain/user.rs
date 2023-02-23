use derive_more::Constructor;
use serde::{Deserialize, Serialize};

pub mod user_repo;

#[derive(Debug, Deserialize, Serialize, Clone, Constructor)]
pub struct User {
    id: String,
    name: String,
}
