use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

pub mod book_repo;

static ID_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]{4}$").unwrap());

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct Book {
    #[validate(regex(path = *ID_PATTERN))]
    id: String,

    #[validate(length(min = 1, max = 100))]
    title: String,
}

impl Book {
    pub fn try_new(id: String, title: String) -> Result<Self, ValidationErrors> {
        let self_ = Self { id, title };
        self_.validate()?;
        Ok(self_)
    }
}
