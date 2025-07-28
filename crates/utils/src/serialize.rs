/// Serialization module for the pairings used by marshmallow.
use thiserror::Error;

pub mod fq;
pub mod point;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("failed to decode base64 data")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("invalid data encountered during deserialization")]
    InvalidData,
}
