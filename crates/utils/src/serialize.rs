/// Serialization module for the pairings used by dcipher.
use thiserror::Error;

pub mod fq;
pub mod point;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("failed to decode base64 data")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("invalid data encountered during deserialization")]
    InvalidData,

    #[error("ark-serialize error: {0}")]
    ArkSerialize(String),
}

impl From<ark_serialize::SerializationError> for SerializationError {
    fn from(value: ark_serialize::SerializationError) -> Self {
        Self::ArkSerialize(value.to_string())
    }
}
