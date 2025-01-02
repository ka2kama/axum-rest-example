pub(crate) mod book_deserializer;
pub(crate) mod user_deserializer;

use aws_sdk_dynamodb::types::AttributeValue;
use im_rc::Vector;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DeserializationError {
    #[error("field mapping error `{0:?}`")]
    MappingError(Vector<MappingFieldError>),
    #[error("field error `{0:?}`")]
    ValidationError(ValidationErrors),
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum MappingFieldError {
    #[error("the data for key `{0}` is missing")]
    MissingKey(String),
    #[error("the data for key `{0}` is `{1:?}`")]
    InvalidType(String, AttributeValue),
}
