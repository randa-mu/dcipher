//! Module for the blocklock agent.

use alloy::primitives::U256;
use alloy::sol_types::SolValue;
use serde::{Deserialize, Serialize};

pub mod agent;
mod condition_resolver;
pub mod contracts;
pub mod fulfiller;

/// Supported blocklock conditions.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BlocklockCondition {
    BlockNumber(BlockNumber),
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct BlockNumber(pub u64);

#[derive(thiserror::Error, Debug)]
pub enum BlocklockConditionDecodeError {
    #[error("missing block number prefix")]
    MissingBlockNumberPrefix,

    #[error("failed to decode ABI encoded type: {1}")]
    AbiDecode(#[source] alloy::sol_types::Error, &'static str),

    #[error("failed to cast block number to u64")]
    BlockNumberToU64(#[from] alloy::primitives::ruint::FromUintError<u64>),
}

impl BlocklockCondition {
    pub fn from_slice(bytes: &[u8]) -> Result<Self, BlocklockConditionDecodeError> {
        let block_number: u64 = U256::abi_decode(bytes, true)
            .map_err(|e| {
                BlocklockConditionDecodeError::AbiDecode(e, "could not decode block number as U256")
            })?
            .try_into()?;
        Ok(BlocklockCondition::BlockNumber(block_number.into()))

        // Implementation with condition prefix, currently not supported by contracts
        // match bytes[0] {
        //     b'B' => {
        //         let block_number: u64 = U256::abi_decode(&bytes[1..], true)
        //             .map_err(|e| {
        //                 BlocklockConditionDecodeError::AbiDecode(
        //                     e,
        //                     "could not decode block number as U256",
        //                 )
        //             })?
        //             .try_into()?;
        //         Ok(BlocklockCondition::BlockNumber(block_number.into()))
        //     }
        //
        //     _ => Err(BlocklockConditionDecodeError::MissingBlockNumberPrefix),
        // }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            BlocklockCondition::BlockNumber(BlockNumber(block_u64)) => {
                U256::from(*block_u64).abi_encode()
                // Implementation with condition prefix, currently not supported by contracts
                // [vec![b'B'], U256::from(*block_u64).abi_encode()].concat()
            }
        }
    }
}

impl TryFrom<&[u8]> for BlocklockCondition {
    type Error = BlocklockConditionDecodeError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(value)
    }
}

impl From<BlocklockCondition> for Vec<u8> {
    fn from(value: BlocklockCondition) -> Self {
        value.to_bytes()
    }
}

impl From<BlocklockCondition> for alloy::primitives::Bytes {
    fn from(value: BlocklockCondition) -> Self {
        let bytes: Vec<u8> = value.into();
        Self::from(bytes)
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
