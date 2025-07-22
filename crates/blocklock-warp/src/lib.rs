//! Blocklock agent implementation for managing blocklock smart contract state
//! and forwarding fulfilled requests to a fulfiller's request channel.

use enc_core::BlockNumber;
use serde::{Deserialize, Serialize};

pub mod agent;
pub mod condition_resolver;
pub mod contracts;
pub mod fulfiller;
pub mod metrics;

// Core types for blocklock conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlocklockCondition {
    BlockNumber(BlockNumber),
}

impl BlocklockCondition {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            BlocklockCondition::BlockNumber(block_number) => {
                let mut bytes = vec![0u8]; // Type discriminator
                bytes.extend_from_slice(&block_number.0.to_be_bytes());
                bytes
            }
        }
    }
}

impl TryFrom<&[u8]> for BlocklockCondition {
    type Error = BlocklockConditionDecodeError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            return Err(BlocklockConditionDecodeError::InvalidFormat);
        }
        
        match bytes[0] {
            0 => {
                if bytes.len() != 9 {
                    return Err(BlocklockConditionDecodeError::InvalidFormat);
                }
                let mut block_bytes = [0u8; 8];
                block_bytes.copy_from_slice(&bytes[1..9]);
                Ok(BlocklockCondition::BlockNumber(BlockNumber(u64::from_be_bytes(block_bytes))))
            }
            _ => Err(BlocklockConditionDecodeError::UnknownType),
        }
    }
}

impl From<BlocklockCondition> for Vec<u8> {
    fn from(condition: BlocklockCondition) -> Self {
        condition.to_bytes()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlocklockConditionDecodeError {
    InvalidFormat,
    UnknownType,
}

impl std::fmt::Display for BlocklockConditionDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlocklockConditionDecodeError::InvalidFormat => write!(f, "Invalid condition format"),
            BlocklockConditionDecodeError::UnknownType => write!(f, "Unknown condition type"),
        }
    }
}

impl std::error::Error for BlocklockConditionDecodeError {}

// Re-export main types
pub use agent::*;
pub use condition_resolver::*;
pub use contracts::*;
pub use fulfiller::*;
pub use metrics::*;
