use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};
use base64::prelude::*;
use serde::{ser::SerializeSeq, Deserialize};

pub fn to_base64<T: CanonicalSerialize>(element: &T) -> Result<String, SerializationError> {
    // Create a buffer to hold the serialized data
    let mut buf = vec![];
    // Serialize the element into the buffer
    element.serialize_with_mode(&mut buf, Compress::Yes)?;
    // Encode the buffer as a base64 string
    Ok(BASE64_STANDARD.encode(&buf))
}

pub fn from_base64<T: CanonicalDeserialize>(str: &str) -> Result<T, SerializationError> {
    let buf = BASE64_STANDARD
        .decode(str)
        .map_err(|_| ark_serialize::SerializationError::InvalidData)?;

    T::deserialize_with_mode(buf.as_slice(), Compress::Yes, Validate::Yes)
}

pub fn ser_vec_base64<S, A: CanonicalSerialize>(arr: &[A], s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = s.serialize_seq(Some(arr.len()))?;
    for element in arr {
        let base64_str = to_base64(element).map_err(serde::ser::Error::custom)?;
        seq.serialize_element(&base64_str)?;
    }
    seq.end()
}

pub fn ser_base64<S, A: CanonicalSerialize>(a: &A, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let base64_str = to_base64(a).map_err(serde::ser::Error::custom)?;
    s.serialize_str(&base64_str)
}

pub fn deser_vec_base64<'de, D, A: CanonicalDeserialize>(
    deserializer: D,
) -> Result<Vec<A>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let base64_vec: Vec<String> = Vec::deserialize(deserializer)?;

    base64_vec
        .into_iter()
        .map(|base64_str| from_base64(&base64_str).map_err(serde::de::Error::custom))
        .collect()
}

pub fn deser_base64<'de, D, A: CanonicalDeserialize>(deserializer: D) -> Result<A, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let base64_str = String::deserialize(deserializer)?;

    from_base64(&base64_str).map_err(serde::de::Error::custom)
}
