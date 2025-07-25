#[cfg(feature = "decryption_sender")]
pub mod decryption_sender;

#[cfg(feature = "signature_sender")]
pub mod signature_sender;

#[cfg(feature = "fulfiller")]
pub mod fulfiller;

// #[cfg(feature = "ibe")]
// pub mod ibe_helper;

#[cfg(feature = "signer")]
pub mod signer {
    pub use dcipher_signer::*;
}

// Re-export contracts from contracts-core
pub mod contracts {
    pub use contracts_core::blocklock::blocklock::blocklock_sender::BlocklockSender;
}

// Re-exports
#[cfg(feature = "decryption_sender")]
pub use decryption_sender::*;

#[cfg(feature = "signature_sender")]
pub use signature_sender::*;

#[cfg(feature = "fulfiller")]
pub use fulfiller::*;

// Common types
#[cfg(feature = "evm")]
use alloy::primitives::U256;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[cfg(feature = "evm")]
mod request_id {
    use super::*;

    /// A unique identifier for a request.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct RequestId(pub U256);

    impl From<U256> for RequestId {
        fn from(value: U256) -> Self {
            Self(value)
        }
    }

    impl From<RequestId> for U256 {
        fn from(value: RequestId) -> Self {
            value.0
        }
    }
}

#[cfg(feature = "evm")]
pub use request_id::RequestId;

/// A block number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockNumber(pub u64);

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

impl Add<u64> for BlockNumber {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<u64> for BlockNumber {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}
