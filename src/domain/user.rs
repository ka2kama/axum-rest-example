use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

pub mod user_repo;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct User {
   #[validate(length(min = 1, max = 10))]
   id: String,

   #[validate(length(min = 1, max = 10))]
   name: String,
}

impl User {
   pub fn try_new(id: String, name: String) -> Result<Self, ValidationErrors> {
      let self_ = Self { id, name };
      self_.validate()?;
      Ok(self_)
   }
}
