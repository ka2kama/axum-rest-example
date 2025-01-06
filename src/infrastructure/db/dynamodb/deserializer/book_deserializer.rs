use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::{
   domain::book::Book,
   infrastructure::db::dynamodb::deserializer::{
      DeserializationError,
      MappingFieldError,
      MappingFieldError::MissingKey,
   },
};

pub fn deserialize_book(
   mut item: HashMap<String, AttributeValue>,
) -> anyhow::Result<Book, DeserializationError> {
   let id = match item.remove_entry("id") {
      Some((_, AttributeValue::S(s))) => Ok(s),
      Some((k, v)) => Err(MappingFieldError::InvalidType(k, v)),
      _ => Err(MissingKey("id".to_string())),
   };

   let name = match item.remove_entry("title") {
      Some((_, AttributeValue::S(s))) => Ok(s),
      Some((k, v)) => Err(MappingFieldError::InvalidType(k, v)),
      _ => Err(MissingKey("title".to_string())),
   };

   match (id, name) {
      (Ok(id), Ok(name)) => Book::try_new(id, name).map_err(DeserializationError::ValidationError),
      (id, name) => {
         let errors = [id.err(), name.err()].into_iter().flatten().collect();
         Err(DeserializationError::MappingError(errors))
      }
   }
}
