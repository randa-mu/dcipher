//! Crate with various helpers for arkworks' pairing such as hashing
//! to elliptic curve, serializing points in a custom format.

/// Helpers for the crate's modules.
mod helpers;

/// Hashing to elliptic curve for Bn254.
pub mod hash_to_curve;

/// Custom serialization for pairings.
pub mod serialize;

/// RFC 9380 DST builder.
pub mod dst;

pub use serde_as::Base64OrBytes;

mod serde_as {
    use serde::{Deserializer, Serializer};
    use serde_with::base64::Standard;
    use serde_with::{formats, DeserializeAs, SerializeAs};

    /// Custom serde_as type that serializes as base64 for human-readable formats (e.g. JSON) and
    /// as raw bytes for binary formats (e.g. cbor).
    pub struct Base64OrBytes;

    impl<T> SerializeAs<T> for Base64OrBytes
    where
        T: AsRef<[u8]>,
        serde_with::Bytes: SerializeAs<T>,
    {
        fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                serde_with::base64::Base64::<Standard, formats::Padded>::serialize_as(
                    source, serializer,
                )
            } else {
                serde_with::Bytes::serialize_as(source, serializer)
            }
        }
    }

    impl<'de, T> DeserializeAs<'de, T> for Base64OrBytes
    where
        T: TryFrom<Vec<u8>>,
        T::Error: std::fmt::Display,
        serde_with::Bytes: DeserializeAs<'de, T>,
    {
        fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                serde_with::base64::Base64::<Standard, formats::Padded>::deserialize_as(
                    deserializer,
                )
            } else {
                serde_with::Bytes::deserialize_as(deserializer)
            }
        }
    }
}
