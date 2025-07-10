//! Module for agent-specific code such as blocklock.

use serde::{Deserialize, Serialize};

#[cfg(feature = "blocklock")]
pub mod blocklock;

#[cfg(feature = "randomness")]
pub mod randomness;

#[cfg(feature = "payment")]
mod payment;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize, Debug,
)]
pub struct RequestId(pub alloy::primitives::U256);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct BlockNumber(pub u64);

impl From<RequestId> for alloy::primitives::U256 {
    fn from(value: RequestId) -> Self {
        value.0
    }
}

impl From<alloy::primitives::U256> for RequestId {
    fn from(value: alloy::primitives::U256) -> Self {
        Self(value)
    }
}

impl AsRef<alloy::primitives::U256> for RequestId {
    fn as_ref(&self) -> &alloy::primitives::U256 {
        &self.0
    }
}

impl From<u64> for BlockNumber {
    fn from(number: u64) -> Self {
        BlockNumber(number)
    }
}

impl From<BlockNumber> for u64 {
    fn from(number: BlockNumber) -> Self {
        number.0
    }
}

impl AsRef<u64> for BlockNumber {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}
