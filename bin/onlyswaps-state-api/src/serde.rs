use alloy::primitives::U256;
use serde::ser::Error;
use serde::{Serialize, Serializer};

// this is a u256 in disguise
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LongNumber(pub U256);

// this is a u64 in disguise
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortNumber(pub U256);

impl Serialize for LongNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl From<U256> for LongNumber {
    fn from(value: U256) -> Self {
        LongNumber(value)
    }
}

impl Serialize for ShortNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0.try_into().map_err(S::Error::custom)?)
    }
}

impl From<U256> for ShortNumber {
    fn from(value: U256) -> Self {
        ShortNumber(value)
    }
}
